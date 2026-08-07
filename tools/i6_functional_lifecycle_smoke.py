#!/usr/bin/env python3
"""Qualify F01-F06 through two clean OCI registries and the real V3 provider.

The caller owns registry process setup. This tool derives current authored
source from the approved A6/A7 examples, removes historical authored Locks,
builds every package through NAPE, publishes the exact build results, resolves
and executes the complete closure, and validates the committed I6 outputs.
"""

from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import urllib.request

import jsonschema


ACTION_PACKAGE = (
    "pkg:attestify/acme.example/verification-action/"
    "release-readiness/database-connection@2.0.1"
)
ACTIVITY_PACKAGE = (
    "pkg:attestify/acme.example/verification-activity/"
    "platform/linux-release-readiness@1.0.0"
)
PROCEDURE_PACKAGE = (
    "pkg:attestify/acme.example/verification-procedure/"
    "release/application-release-readiness@2.0.0"
)
PACKAGES = (ACTION_PACKAGE, ACTIVITY_PACKAGE, PROCEDURE_PACKAGE)
CURRENT_I6_OPAQUE_TEST_FIELDS = {
    "artifact-integrity.py": ("observed", "digest_matches"),
    "audit-logging.py": ("observed", "enabled"),
    "disk-encryption.py": ("observed", "encrypted"),
    "host-baseline.py": ("status", "baseline_status"),
    "time-synchronization.py": ("observed", "synchronized"),
}
ENGINE_PROJECTION_DIGEST = (
    "sha256:cb35b842d37063028ca89435cb6d136f3eeacc25bab4dbea20e435bf5c5592e9"
)
EVALUATOR_PROJECTION_DIGEST = (
    "sha256:8885a6be40b99540dc0d2662b011ccccee0e69055a330e63b700ac06efa69deb"
)


def canonical(value: object) -> bytes:
    return json.dumps(
        value, allow_nan=False, sort_keys=True, separators=(",", ":")
    ).encode()


def digest(data: bytes) -> str:
    return "sha256:" + hashlib.sha256(data).hexdigest()


def tree_digest(root: Path, suffix: str | None = None) -> str:
    calculation = hashlib.sha256()
    files = sorted(
        path
        for path in root.rglob("*")
        if path.is_file() and (suffix is None or path.suffix == suffix)
    )
    for path in files:
        relative = path.relative_to(root).as_posix().encode()
        raw = path.read_bytes()
        calculation.update(relative)
        calculation.update(b"\0")
        calculation.update(len(raw).to_bytes(8, "big"))
        calculation.update(hashlib.sha256(raw).digest())
    return "sha256:" + calculation.hexdigest()


def canonical_port(name: str, default: str) -> int:
    value = os.environ.get(name, default)
    if (
        not value.isascii()
        or not value.isdecimal()
        or value.startswith("0")
        or not 1 <= int(value) <= 65535
    ):
        raise RuntimeError(f"{name} must be a canonical TCP port")
    return int(value)


def validator(bundle: dict, selector: str) -> jsonschema.Draft202012Validator:
    return jsonschema.Draft202012Validator(
        {"$schema": bundle["$schema"], "$defs": bundle["$defs"], "$ref": selector}
    )


def current_i6_test_source(name: str, source: str) -> str:
    """Translate only the approved conceptual A7 opaque-Test source forms."""
    translated = source.replace('"value_type": "null",', '"value_type": None,')
    field = CURRENT_I6_OPAQUE_TEST_FIELDS.get(name)
    if field is None:
        return translated

    variable, key = field
    historical_read = (
        f'        {variable} = json.loads(evidence.decode("utf-8"))["{key}"]'
    )
    current_read = (
        '        decoded = json.loads('
        'evidence.decode("utf-8", errors="strict")'
        ')\n'
        f'        {variable} = decoded.get("{key}") if isinstance(decoded, dict) else None'
    )
    historical_handler = (
        "    except (UnicodeDecodeError, json.JSONDecodeError, KeyError, TypeError):"
    )
    current_handler = (
        "    except (UnicodeDecodeError, json.JSONDecodeError, TypeError):"
    )
    if translated.count(historical_read) != 1:
        raise RuntimeError(f"unexpected historical evidence read in {name}")
    if translated.count(historical_handler) != 1:
        raise RuntimeError(f"unexpected historical exception handler in {name}")
    translated = translated.replace(historical_read, current_read, 1)
    translated = translated.replace(historical_handler, current_handler, 1)

    if name == "host-baseline.py":
        historical_invalid = """    except (UnicodeDecodeError, json.JSONDecodeError, TypeError):
        return {
            "conclusion": "inconclusive",
            "facts": [
                {
                    "name": "baseline_status",
                    "value": None,
                    "value_type": None,
                    "status": "invalid",
                }
            ],
            "reason": (
                "A valid host baseline status was not available, so the claim "
                "cannot be evaluated."
            ),
        }
"""
        current_invalid = """    except (UnicodeDecodeError, json.JSONDecodeError, TypeError):
        status = None
    if not isinstance(status, str):
        return {
            "conclusion": "inconclusive",
            "facts": [
                {
                    "name": "baseline_status",
                    "value": None,
                    "value_type": None,
                    "status": "invalid",
                }
            ],
            "reason": (
                "A valid host baseline status was not available, so the claim "
                "cannot be evaluated."
            ),
        }
"""
        if translated.count(historical_invalid) != 1:
            raise RuntimeError("unexpected historical host-baseline invalid branch")
        translated = translated.replace(historical_invalid, current_invalid, 1)

    if 'decode("utf-8")' in translated or "KeyError" in translated:
        raise RuntimeError(f"current source retained a historical form in {name}")
    return translated


class Qualification:
    def __init__(self, workspace: Path, root: Path) -> None:
        self.workspace = workspace
        self.root = root
        self.nape = Path(
            os.environ.get(
                "NAPE_I6_BINARY", workspace / "applications/nape/target/debug/nape"
            )
        )
        self.evaluator = workspace / "applications/nape-evaluator"
        self.examples = workspace / (
            "1-attestify-product-workspace-docs/0-plans/attestify-oci/"
            "attestify-oci-repository-profile/q12-verification-action-capability-ladder/"
            "examples"
        )
        self.contracts = workspace / (
            "applications/nape/contracts/"
            "attestify-nape-verification-engine-contract-projection-v2"
        )
        if not self.nape.is_file():
            raise RuntimeError("build applications/nape/target/debug/nape first")
        if not (self.evaluator / "main.py").is_file():
            raise RuntimeError("the sibling NAPE Evaluator source tree is absent")
        projection_check = subprocess.run(
            ["python3", str(self.contracts / "verify_projection.py")],
            check=False,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        if (
            projection_check.returncode != 0
            or ENGINE_PROJECTION_DIGEST.encode() not in projection_check.stdout
        ):
            raise RuntimeError("the installed NAPE I6 projection is not the frozen projection")
        evaluator_projection_check = subprocess.run(
            [
                "python3",
                str(
                    self.evaluator
                    / "contracts/attestify-nape-evaluator-action-invocation-v3/verify_projection.py"
                ),
            ],
            check=False,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        if (
            evaluator_projection_check.returncode != 0
            or EVALUATOR_PROJECTION_DIGEST.encode()
            not in evaluator_projection_check.stdout
        ):
            raise RuntimeError(
                "the installed Evaluator V3 projection is not the frozen projection"
            )
        bundle = json.loads(
            (
                self.contracts
                / "verification-engine/i0-i6/verification-engine-contracts.schema.json"
            ).read_bytes()
        )
        receipt_bundle = json.loads(
            (
                self.contracts
                / "command-receipt/v2/nape-command-receipt.schema.json"
            ).read_bytes()
        )
        self.receipt_validator = validator(receipt_bundle, "#/$defs/napeCommandReceiptV2")
        self.report_validator = validator(bundle, "#/$defs/verificationReportI6")
        self.relationship_validator = validator(
            bundle, "#/$defs/evidenceSetRelationshipI6"
        )
        self.sources = self.prepare_sources()
        self.builds = {
            package: root / "build" / str(index)
            for index, package in enumerate(PACKAGES)
        }
        self.manifests: dict[str, str] = {}
        self.environment = self.real_evaluator_environment()

    def prepare_sources(self) -> dict[str, Path]:
        source_root = self.root / "source"
        a6 = self.examples / "a6-feature-rich-profile-1-action/packages/action"
        a7 = self.examples / "a7-feature-rich-profile-1-procedure/packages"
        candidates = {
            ACTION_PACKAGE: (a6, "verification-action.yaml"),
            ACTIVITY_PACKAGE: (a7 / "activity", "verification-activity.yaml"),
            PROCEDURE_PACKAGE: (a7 / "procedure", "verification-procedure.yaml"),
        }
        prepared: dict[str, Path] = {}
        translated_opaque_tests: set[str] = set()
        for index, (package, (source, semantic_root)) in enumerate(candidates.items()):
            destination = source_root / str(index)
            shutil.copytree(
                source,
                destination,
                ignore=shutil.ignore_patterns("attestify-lock.json", "__pycache__"),
            )
            document = destination / semantic_root
            text = document.read_text(encoding="utf-8")
            if not text.startswith("apiVersion: 2."):
                raise RuntimeError(f"unexpected historical apiVersion in {semantic_root}")
            document.write_text(
                "apiVersion: 2.0.0\n" + text.split("\n", 1)[1], encoding="utf-8"
            )
            for test in destination.rglob("*.py"):
                current = current_i6_test_source(
                    test.name, test.read_text(encoding="utf-8")
                )
                if test.name in CURRENT_I6_OPAQUE_TEST_FIELDS:
                    if test.name in translated_opaque_tests:
                        raise RuntimeError(
                            f"duplicate conceptual A7 Test source: {test.name}"
                        )
                    translated_opaque_tests.add(test.name)
                test.write_text(current, encoding="utf-8")
            if (destination / "attestify-lock.json").exists():
                raise RuntimeError("current authored source retained a historical Lock")
            prepared[package] = destination
        if translated_opaque_tests != set(CURRENT_I6_OPAQUE_TEST_FIELDS):
            missing = sorted(set(CURRENT_I6_OPAQUE_TEST_FIELDS) - translated_opaque_tests)
            raise RuntimeError(f"conceptual A7 Test source is absent: {missing}")
        return prepared

    def real_evaluator_environment(self) -> dict[str, str]:
        configured_wrapper = os.environ.get("NAPE_I6_EVALUATOR")
        if configured_wrapper is None:
            wrapper = self.root / "nape-eval-v3"
            wrapper.write_text(
                "#!/bin/sh\n"
                f"export PYTHONPATH='{self.evaluator / 'src'}'\n"
                f"exec python3 '{self.evaluator / 'main.py'}' \"$@\"\n",
                encoding="utf-8",
            )
            wrapper.chmod(0o700)
        else:
            wrapper = Path(configured_wrapper)
            if not wrapper.is_absolute() or not wrapper.is_file() or not os.access(wrapper, os.X_OK):
                raise RuntimeError("NAPE_I6_EVALUATOR must name an executable absolute path")
        implementation = tree_digest(self.evaluator / "src/nape_evaluator", ".py")
        dependencies = digest((self.evaluator / "pyproject.toml").read_bytes())
        record = canonical(
            {
                "contract": "attestify.nape-evaluator.installed-build/v1",
                "contract_projection_digest": EVALUATOR_PROJECTION_DIGEST,
                "dependency_set_digest": dependencies,
                "evaluator_contract": "attestify.nape-evaluator.action-invocation/v3",
                "evaluator_release": "2.0.0",
                "implementation_digest": implementation,
                "runner_profile": "attestify-python-test-development-v2",
            }
        )
        record_path = self.root / "evaluator-v3-build.json"
        record_path.write_bytes(record)
        environment = dict(os.environ)
        environment.update(
            {
                "NAPE_EVALUATOR_V3_EXECUTABLE": str(wrapper),
                "NAPE_EVALUATOR_V3_BUILD_RECORD": str(record_path),
                "NAPE_EVALUATOR_V3_BUILD_RECORD_SHA256": digest(record),
            }
        )
        self.evaluator_identities = {
            "buildRecord": digest(record),
            "contractProjection": EVALUATOR_PROJECTION_DIGEST,
            "dependencySet": dependencies,
            "implementation": implementation,
        }
        return environment

    def command(self, arguments: list[str]) -> dict:
        completed = subprocess.run(
            [str(self.nape), *arguments],
            check=False,
            env=self.environment,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        if completed.returncode != 0:
            raise RuntimeError(
                f"NAPE failed for {arguments}: {completed.stderr.decode(errors='replace')}"
            )
        if completed.stdout.count(b"\n") != 1 or not completed.stdout.endswith(b"\n"):
            raise RuntimeError(f"NAPE did not emit one framed Receipt: {arguments}")
        receipt = json.loads(completed.stdout)
        if canonical(receipt) != completed.stdout[:-1]:
            raise RuntimeError(f"NAPE Receipt is not canonical JSON: {arguments}")
        self.receipt_validator.validate(receipt)
        return receipt

    def build_packages(self) -> None:
        (self.root / "build").mkdir()
        dependencies = {
            ACTION_PACKAGE: (),
            ACTIVITY_PACKAGE: (ACTION_PACKAGE,),
            PROCEDURE_PACKAGE: (ACTIVITY_PACKAGE, ACTION_PACKAGE),
        }
        for package in PACKAGES:
            arguments = [
                "package",
                "build",
                "--source",
                str(self.sources[package]),
                "--package",
                package,
            ]
            for dependency in dependencies[package]:
                arguments.extend(["--dependency-package", str(self.builds[dependency])])
            arguments.extend(["--output", str(self.builds[package])])
            result = self.command(arguments)["result"]
            if result["package"] != package:
                raise RuntimeError("build Receipt changed package identity")
            self.manifests[package] = result["manifestDigest"]

        rebuild_root = self.root / "rebuild"
        rebuild_root.mkdir()
        rebuilds: dict[str, Path] = {}
        for index, package in enumerate(PACKAGES):
            output = rebuild_root / str(index)
            arguments = [
                "package",
                "build",
                "--source",
                str(self.sources[package]),
                "--package",
                package,
            ]
            for dependency in dependencies[package]:
                arguments.extend(["--dependency-package", str(rebuilds[dependency])])
            arguments.extend(["--output", str(output)])
            result = self.command(arguments)["result"]
            if result["manifestDigest"] != self.manifests[package]:
                raise RuntimeError(f"two builds changed identity for {package}")
            for name in ("manifest.json", "config.json", "package.tar"):
                if (output / name).read_bytes() != (self.builds[package] / name).read_bytes():
                    raise RuntimeError(f"two builds changed {name} for {package}")
            rebuilds[package] = output

    def registry_map(self, port: int) -> Path:
        path = self.root / f"registry-map-{port}.yaml"
        path.write_text(
            "profileVersion: attestify-oci-registry-map/1\n"
            "publishers:\n"
            "  acme.example:\n"
            "    scheme: http\n"
            f"    registry: localhost:{port}\n"
            "    repositoryPrefix: attestify\n",
            encoding="utf-8",
        )
        return path

    @staticmethod
    def require_clean_registry(port: int) -> None:
        with urllib.request.urlopen(f"http://localhost:{port}/v2/", timeout=10) as response:
            if response.status != 200:
                raise RuntimeError(f"registry on port {port} is unavailable")
        catalog = json.loads(
            urllib.request.urlopen(f"http://localhost:{port}/v2/_catalog", timeout=10).read()
        )
        if catalog.get("repositories") != []:
            raise RuntimeError(f"registry on port {port} is not clean")

    def publish_and_resolve(self, registry_map: Path) -> None:
        for package in PACKAGES:
            result = self.command(
                [
                    "package",
                    "publish",
                    "--local-package",
                    str(self.builds[package]),
                    "--registry-profile",
                    str(registry_map),
                ]
            )["result"]
            if result["manifestDigest"] != self.manifests[package]:
                raise RuntimeError("publication changed a package manifest")
        plan = self.command(
            [
                "package",
                "resolve",
                "--package",
                PROCEDURE_PACKAGE,
                "--manifest-digest",
                self.manifests[PROCEDURE_PACKAGE],
                "--registry-profile",
                str(registry_map),
                "--plan-only",
            ]
        )["result"]
        dependencies = {row["package"] for row in plan["dependency"]}
        if dependencies != {ACTION_PACKAGE, ACTIVITY_PACKAGE}:
            raise RuntimeError("resolve plan did not expose the exact two-package closure")

    def verify(self, registry_map: Path, name: str) -> tuple[dict, dict]:
        evidence = self.examples / "a7-feature-rich-profile-1-procedure/evidence"
        subject = self.root / "subject.json"
        if not subject.exists():
            subject.write_bytes(
                canonical({"arn": "risk-source:01KWJK2M4W6AX3XJ7C0Q8N5R2T"})
            )
        output = self.root / "result" / name
        output.parent.mkdir(exist_ok=True)
        receipt = self.command(
            [
                "verify",
                "--package",
                PROCEDURE_PACKAGE,
                "--manifest-digest",
                self.manifests[PROCEDURE_PACKAGE],
                "--registry-profile",
                str(registry_map),
                "--evidence-root",
                str(evidence),
                "--subject-file",
                str(subject),
                "--output",
                str(output),
            ]
        )
        report = json.loads((output / "verification-report.json").read_bytes())
        relationship = json.loads(
            (output / "evidence-set-relationship.json").read_bytes()
        )
        self.report_validator.validate(report)
        self.relationship_validator.validate(relationship)
        if receipt["result"]["report"]["summary"] != report["summary"]:
            raise RuntimeError("Receipt and committed Report summaries differ")
        if report["summary"]["action_count"] != 7:
            raise RuntimeError("feature-rich Procedure did not execute seven occurrences")
        if report["summary"]["actions_completed"] != 7:
            raise RuntimeError("real V3 provider did not complete every occurrence")
        if report["summary"]["conclusion_true"] < 1:
            raise RuntimeError("feature-rich result omitted true conclusions")
        if report["summary"]["conclusion_false"] < 1:
            raise RuntimeError("feature-rich result omitted false conclusions")
        if report["summary"]["conclusion_inconclusive"] < 1:
            raise RuntimeError("feature-rich result omitted inconclusive conclusions")
        rows = {
            action["action"]: action
            for activity in report["activity"]
            for action in activity["action"]
        }
        primary = rows["infrastructure-readiness.primary-database"]
        recovery = rows["release-readiness.disaster-recovery-database"]
        for row in (primary, recovery):
            if row["evidence"]["declaration"]["representation"] != "validated-structured":
                raise RuntimeError("schema-selected evidence was not structured")
            if len(row["test"]["resolved_module"]) != 2:
                raise RuntimeError("declared helper closure was not reported")
            if row["test"]["implementation"]["evaluator"]["id"] != "nape-evaluator-v3":
                raise RuntimeError("functional occurrence did not select Evaluator V3")
        primary_endpoint = primary["evaluations"][2]
        recovery_endpoint = recovery["evaluations"][2]
        if primary_endpoint["criterion_origin"]["equals"]["type"] != "package-default":
            raise RuntimeError("package-default criterion origin changed")
        if recovery_endpoint["criterion_origin"]["equals"]["type"] != "parent-assignment":
            raise RuntimeError("authorized parent-assignment origin changed")
        if len(relationship["occurrence"]) != 7:
            raise RuntimeError("Evidence Set omitted an Action occurrence")
        for payload in relationship["payload"]:
            stored = output / "evidence/sha256" / payload["content_digest"].removeprefix("sha256:")
            if not stored.is_file() or digest(stored.read_bytes()) != payload["content_digest"]:
                raise RuntimeError("digest-addressed raw evidence is absent or changed")
        return report, relationship

    @staticmethod
    def normalized(report: dict, relationship: dict) -> tuple[dict, dict]:
        report = json.loads(json.dumps(report))
        relationship = json.loads(json.dumps(relationship))
        report["invocation"]["id"] = "<invocation-id>"
        report["metadata"]["id"] = "<report-id>"
        report["metadata"]["generated_at"] = "<generated-at>"
        relationship["report"]["metadata"]["id"] = "<report-id>"
        return report, relationship


def main() -> None:
    workspace = Path(
        os.environ.get("ATTESTIFY_WORKSPACE_ROOT", Path(__file__).resolve().parents[3])
    )
    first_port = canonical_port("ATTESTIFY_I6_REGISTRY_PORT_ONE", "51245")
    second_port = canonical_port("ATTESTIFY_I6_REGISTRY_PORT_TWO", "51246")
    if first_port == second_port:
        raise RuntimeError("I6 qualification requires two distinct registry ports")
    Qualification.require_clean_registry(first_port)
    Qualification.require_clean_registry(second_port)
    with tempfile.TemporaryDirectory(prefix="attestify-i6-lifecycle-") as temporary:
        qualification = Qualification(workspace, Path(temporary))
        qualification.build_packages()
        first_map = qualification.registry_map(first_port)
        second_map = qualification.registry_map(second_port)
        qualification.publish_and_resolve(first_map)
        first_report, first_relationship = qualification.verify(first_map, "first")
        qualification.publish_and_resolve(second_map)
        second_report, second_relationship = qualification.verify(second_map, "second")
        if qualification.normalized(first_report, first_relationship) != qualification.normalized(
            second_report, second_relationship
        ):
            raise RuntimeError("functional observations differ across clean registries")
        result = {
            "contract": "attestify.nape.i6-functional-lifecycle-qualification/v1",
            "evaluatorIdentities": qualification.evaluator_identities,
            "manifestDigests": qualification.manifests,
            "proofs": {
                "cleanRegistryRuns": 2,
                "deterministicBuilds": 2,
                "effectiveOccurrences": 7,
                "localPersistence": "passed",
                "realEvaluatorV3": "passed",
                "semanticEquality": "passed",
            },
            "status": "passed",
        }
        print(canonical(result).decode())


if __name__ == "__main__":
    main()
