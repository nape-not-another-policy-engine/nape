#!/usr/bin/env python3
"""Run the checked minimal and complete Verification V2 OCI lifecycles."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import urllib.parse
import urllib.request

import jsonschema


MINIMAL_PACKAGE = (
    "pkg:attestify/acme.example/verification-procedure/"
    "release-readiness/database-endpoint@1.0.0"
)
ACTION_PACKAGE = (
    "pkg:attestify/acme.example/verification-action/"
    "release-readiness/database-connection@2.0.0"
)
ACTIVITY_PACKAGE = (
    "pkg:attestify/acme.example/verification-activity/"
    "release-readiness/database-readiness@2.0.0"
)
PROCEDURE_PACKAGE = (
    "pkg:attestify/acme.example/verification-procedure/"
    "release-readiness/release-readiness@2.0.0"
)
V2_PROJECTION = (
    "sha256:6ac80bec8e158d38f321e4501d6cb749b39e518c823efcc45c86c2e4b7d2b20d"
)
V3_PROJECTION = (
    "sha256:2594b547fb082abf442f6f7cb61e42e9703f3f3feb0415406cd89239aa0d16b7"
)


def canonical(value: object) -> bytes:
    return json.dumps(
        value, allow_nan=False, sort_keys=True, separators=(",", ":")
    ).encode()


def digest(raw: bytes) -> str:
    return "sha256:" + hashlib.sha256(raw).hexdigest()


def tree_digest(root: Path, suffix: str | None = None) -> str:
    calculation = hashlib.sha256()
    for path in sorted(candidate for candidate in root.rglob("*") if candidate.is_file()):
        if suffix is not None and path.suffix != suffix:
            continue
        relative = path.relative_to(root).as_posix().encode()
        raw = path.read_bytes()
        calculation.update(relative)
        calculation.update(b"\0")
        calculation.update(len(raw).to_bytes(8, "big"))
        calculation.update(hashlib.sha256(raw).digest())
    return "sha256:" + calculation.hexdigest()


def endpoint(value: str) -> str:
    parsed = urllib.parse.urlsplit(value)
    if (
        parsed.scheme != "http"
        or parsed.hostname != "localhost"
        or parsed.port is None
        or parsed.username is not None
        or parsed.password is not None
        or parsed.path
        or parsed.query
        or parsed.fragment
        or str(parsed.port) != parsed.netloc.rsplit(":", 1)[-1]
    ):
        raise argparse.ArgumentTypeError(
            "endpoint must be exact http://localhost:<canonical-port>"
        )
    return value


def validator(bundle: dict, selector: str) -> jsonschema.Draft202012Validator:
    return jsonschema.Draft202012Validator(
        {"$schema": bundle["$schema"], "$defs": bundle["$defs"], "$ref": selector}
    )


class Lifecycle:
    def __init__(self, workspace: Path, registry_endpoint: str, root: Path) -> None:
        self.workspace = workspace
        self.endpoint = registry_endpoint
        self.root = root
        (self.root / "build").mkdir()
        (self.root / "result").mkdir()
        self.nape = Path(
            os.environ.get("NAPE_V2_BINARY", workspace / "applications/nape/target/debug/nape")
        )
        self.examples = workspace / (
            "applications/nape/docs/examples/verification-v2-oci"
        )
        self.evaluator = workspace / "applications/nape-evaluator"
        self.contracts = workspace / (
            "applications/nape/contracts/"
            "attestify-nape-verification-engine-contract-projection-v2"
        )
        if not self.nape.is_file():
            raise RuntimeError("build applications/nape/target/debug/nape first")
        if not (self.evaluator / "main.py").is_file():
            raise RuntimeError("the sibling NAPE Evaluator source is absent")

        bundle = json.loads(
            (
                self.contracts
                / "verification-engine/i0-i6/verification-engine-contracts.schema.json"
            ).read_bytes()
        )
        contract_index = json.loads(
            (
                self.contracts
                / "verification-engine/i0-i6/verification-engine-contract-index.json"
            ).read_bytes()
        )
        receipt_bundle = json.loads(
            (
                self.contracts
                / "command-receipt/v2/nape-command-receipt.schema.json"
            ).read_bytes()
        )
        self.receipt_validator = validator(
            receipt_bundle, "#/$defs/napeCommandReceiptV2"
        )
        self.report_validator = validator(
            bundle, contract_index["verification-report-v2"]
        )
        self.relationship_validator = validator(
            bundle,
            contract_index["verification-evidence-set-relationship-v2"],
        )
        self.environment = self.evaluator_environment()

    def evaluator_environment(self) -> dict[str, str]:
        configured = os.environ.get("NAPE_V2_EVALUATOR")
        if configured is None:
            wrapper = self.root / "nape-eval"
            wrapper.write_text(
                "#!/bin/sh\n"
                f"export PYTHONPATH='{self.evaluator / 'src'}'\n"
                f"exec python3 '{self.evaluator / 'main.py'}' \"$@\"\n",
                encoding="utf-8",
            )
            wrapper.chmod(0o700)
        else:
            wrapper = Path(configured)
            if not wrapper.is_absolute() or not wrapper.is_file() or not os.access(wrapper, os.X_OK):
                raise RuntimeError("NAPE_V2_EVALUATOR must be an executable absolute path")

        implementation = tree_digest(self.evaluator / "src/nape_evaluator", ".py")
        dependencies = digest((self.evaluator / "pyproject.toml").read_bytes())
        environment = dict(os.environ)
        for version, projection, runner in (
            ("2", V2_PROJECTION, "attestify-python-test-development-v1"),
            ("3", V3_PROJECTION, "attestify-python-test-development-v2"),
        ):
            contract = f"attestify.nape-evaluator.action-invocation/v{version}"
            record = canonical(
                {
                    "contract": "attestify.nape-evaluator.installed-build/v1",
                    "contract_projection_digest": projection,
                    "dependency_set_digest": dependencies,
                    "evaluator_contract": contract,
                    "evaluator_release": "2.0.0",
                    "implementation_digest": implementation,
                    "runner_profile": runner,
                }
            )
            record_path = self.root / f"evaluator-v{version}-build.json"
            record_path.write_bytes(record)
            environment.update(
                {
                    f"NAPE_EVALUATOR_V{version}_EXECUTABLE": str(wrapper),
                    f"NAPE_EVALUATOR_V{version}_BUILD_RECORD": str(record_path),
                    f"NAPE_EVALUATOR_V{version}_BUILD_RECORD_SHA256": digest(record),
                }
            )
        return environment

    def require_clean_registry(self) -> None:
        try:
            with urllib.request.urlopen(
                f"{self.endpoint}/v2/_catalog", timeout=3
            ) as response:
                catalog = json.load(response)
        except Exception as error:
            raise RuntimeError(f"registry endpoint is unavailable: {error}") from error
        if catalog.get("repositories", []) != []:
            raise RuntimeError("the lifecycle proof requires a clean registry")

    def command(self, arguments: list[str], home: Path | None = None) -> dict:
        environment = dict(self.environment)
        if home is not None:
            environment["HOME"] = str(home)
        completed = subprocess.run(
            [str(self.nape), *arguments],
            check=False,
            env=environment,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        if completed.returncode != 0:
            raise RuntimeError(
                f"NAPE failed ({completed.returncode}): "
                f"stderr={completed.stderr.decode(errors='replace')}; "
                f"stdout={completed.stdout.decode(errors='replace')}"
            )
        if completed.stdout.count(b"\n") != 1 or not completed.stdout.endswith(b"\n"):
            raise RuntimeError("NAPE did not emit one line-feed-framed Receipt")
        receipt = json.loads(completed.stdout)
        if canonical(receipt) != completed.stdout[:-1]:
            raise RuntimeError("NAPE Receipt is not canonical JCS")
        self.receipt_validator.validate(receipt)
        if receipt["producer"].get("qualification") != (
            "attestify-verification-engine-v2-development"
        ):
            raise RuntimeError("NAPE emitted a stale producer qualification")
        return receipt

    def silent_command(self, arguments: list[str], home: Path) -> None:
        environment = dict(self.environment)
        environment["HOME"] = str(home)
        completed = subprocess.run(
            [str(self.nape), *arguments],
            check=False,
            env=environment,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        if completed.returncode != 0:
            raise RuntimeError(
                f"NAPE failed ({completed.returncode}): "
                f"{completed.stderr.decode(errors='replace')}"
            )
        if completed.stdout:
            raise RuntimeError("Start and Evidence success must be silent")

    def build_twice(
        self, source: Path, package: str, dependencies: list[Path], name: str
    ) -> tuple[Path, dict]:
        results = []
        receipts = []
        for repetition in ("a", "b"):
            output = self.root / "build" / f"{name}-{repetition}"
            arguments = [
                "package", "build", "--source", str(source), "--package", package,
            ]
            for dependency in dependencies:
                arguments.extend(["--dependency-package", str(dependency)])
            arguments.extend(["--output", str(output)])
            receipts.append(self.command(arguments))
            results.append(output)
        for filename in ("config.json", "manifest.json", "package.tar"):
            if results[0].joinpath(filename).read_bytes() != results[1].joinpath(filename).read_bytes():
                raise RuntimeError(f"nondeterministic package output: {name}/{filename}")
        normalized_receipts = []
        for receipt in receipts:
            result = dict(receipt["result"])
            result.pop("output", None)
            normalized_receipts.append(result)
        if normalized_receipts[0] != normalized_receipts[1]:
            raise RuntimeError(f"nondeterministic build Receipt result: {name}")
        return results[0], receipts[0]

    def publish(self, package: Path) -> dict:
        return self.command(
            [
                "package", "publish", "--local-package", str(package),
                "--registry-endpoint", self.endpoint,
            ]
        )

    def resolve(self, package: str, manifest: str) -> dict:
        return self.command(
            [
                "package", "resolve", "--package", package,
                "--manifest-digest", manifest, "--registry-endpoint", self.endpoint,
                "--plan-only",
            ]
        )

    @staticmethod
    def normalized(value: dict) -> dict:
        result = json.loads(json.dumps(value))
        if result.get("kind") == "VerificationReport":
            result["invocation"]["id"] = "<invocation-id>"
            result["metadata"]["id"] = "<report-id>"
            result["metadata"]["generated_at"] = "<generated-at>"
            result["metadata"]["utc-start"] = "<utc-start>"
        else:
            result["report"]["metadata"]["id"] = "<report-id>"
        return result

    @staticmethod
    def local_output(receipt: dict) -> Path:
        location = urllib.parse.urlsplit(receipt["result"]["output"])
        if location.scheme != "file" or location.netloc:
            raise RuntimeError("Verify Receipt output is not a canonical local file URI")
        return Path(urllib.parse.unquote(location.path))

    def verify_twice(
        self,
        name: str,
        package: str,
        manifest: str,
        evidence: Path,
        subject: Path,
    ) -> dict:
        observations = []
        for repetition in ("a", "b"):
            home = self.root / "result" / f"{name}-{repetition}-home"
            home.mkdir()
            self.silent_command(
                [
                    "start", "--package", package, "--manifest-digest", manifest,
                    "--registry-endpoint", self.endpoint, "--subject-file", str(subject),
                ],
                home,
            )
            evidence_files = sorted(path for path in evidence.rglob("*") if path.is_file())
            if not evidence_files:
                raise RuntimeError("the staged lifecycle has no evidence inputs")
            for evidence_file in evidence_files:
                relative = evidence_file.relative_to(evidence)
                if len(relative.parts) != 3:
                    raise RuntimeError("evidence must use activity/action/file layout")
                action = f"{relative.parts[0]}.{relative.parts[1]}"
                self.silent_command(
                    [
                        "evidence", "--action", action, "--file", str(evidence_file),
                        "--file-name", relative.parts[2],
                    ],
                    home,
                )
            receipt = self.command(["verify"], home)
            output = self.local_output(receipt)
            report = json.loads((output / "verification-report.json").read_bytes())
            relationship = json.loads(
                (output / "evidence-set-relationship.json").read_bytes()
            )
            self.report_validator.validate(report)
            self.relationship_validator.validate(relationship)
            runners = {
                row["test"]["runner_profile"]["id"]
                for activity in report["activity"]
                for row in activity["action"]
            }
            expected_runner = {
                "minimal": "attestify-python-test-development-v1",
                "complete": "attestify-python-test-development-v2",
            }[name]
            if runners != {expected_runner}:
                raise RuntimeError(
                    f"{name} Report did not use its exact whole-Report Runner mode"
                )
            if receipt["result"]["report"]["summary"] != report["summary"]:
                raise RuntimeError("Receipt and committed Report summaries differ")
            evidence_files = sorted(
                path for path in (output / "evidence/sha256").iterdir() if path.is_file()
            )
            if not evidence_files:
                raise RuntimeError("no raw evidence was committed")
            for path in evidence_files:
                if path.name != digest(path.read_bytes()).removeprefix("sha256:"):
                    raise RuntimeError("raw evidence path does not equal its digest")
            observations.append(
                (self.normalized(report), self.normalized(relationship))
            )
        if observations[0] != observations[1]:
            raise RuntimeError(f"clean {name} executions differ outside generated values")
        return observations[0][0]

    def run(self) -> dict:
        self.require_clean_registry()

        minimal, minimal_build = self.build_twice(
            self.examples / "minimal/source", MINIMAL_PACKAGE, [], "minimal"
        )
        self.publish(minimal)
        self.resolve(MINIMAL_PACKAGE, minimal_build["result"]["manifestDigest"])
        minimal_report = self.verify_twice(
            "minimal",
            MINIMAL_PACKAGE,
            minimal_build["result"]["manifestDigest"],
            self.examples / "minimal/evidence",
            self.examples / "minimal/subject.json",
        )

        action, action_build = self.build_twice(
            self.examples / "complete/source/action", ACTION_PACKAGE, [], "action"
        )
        activity, activity_build = self.build_twice(
            self.examples / "complete/source/activity",
            ACTIVITY_PACKAGE,
            [action],
            "activity",
        )
        procedure, procedure_build = self.build_twice(
            self.examples / "complete/source/procedure",
            PROCEDURE_PACKAGE,
            [action, activity],
            "procedure",
        )
        for package in (action, activity, procedure):
            self.publish(package)
        self.resolve(PROCEDURE_PACKAGE, procedure_build["result"]["manifestDigest"])
        complete_report = self.verify_twice(
            "complete",
            PROCEDURE_PACKAGE,
            procedure_build["result"]["manifestDigest"],
            self.examples / "complete/evidence",
            self.examples / "complete/subject.json",
        )

        return {
            "contract": "attestify.nape.verification-v2-lifecycle-smoke/v1",
            "registryEndpoint": self.endpoint,
            "packages": {
                "minimal": minimal_build["result"]["manifestDigest"],
                "action": action_build["result"]["manifestDigest"],
                "activity": activity_build["result"]["manifestDigest"],
                "procedure": procedure_build["result"]["manifestDigest"],
            },
            "results": {
                "minimal": minimal_report["summary"],
                "complete": complete_report["summary"],
            },
            "status": "passed",
        }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--registry-endpoint",
        type=endpoint,
        default="http://localhost:51247",
    )
    arguments = parser.parse_args()
    workspace = Path(
        os.environ.get(
            "ATTESTIFY_WORKSPACE_ROOT", Path(__file__).resolve().parents[3]
        )
    ).resolve()
    temporary_root = Path(os.environ.get("NAPE_V2_TEMP_ROOT", "/private/tmp"))
    if not temporary_root.is_absolute() or not temporary_root.is_dir():
        raise RuntimeError("NAPE_V2_TEMP_ROOT must be an existing absolute directory")
    with tempfile.TemporaryDirectory(
        prefix="attestify-v2-lifecycle-", dir=temporary_root
    ) as temporary:
        result = Lifecycle(workspace, arguments.registry_endpoint, Path(temporary)).run()
    print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
