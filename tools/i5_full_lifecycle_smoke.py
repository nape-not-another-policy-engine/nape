#!/usr/bin/env python3
"""Qualify the bounded I5 L0-L2 lifecycle against two clean local registries.

The caller owns registry process setup.  This tool uses only NAPE commands for
package construction, publication, planning, and execution; validates every
command receipt; validates every committed Report and Evidence Set
relationship; and compares deterministic L2 observations across registries.
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


L0_PACKAGE = (
    "pkg:attestify/acme.example/verification-procedure/"
    "release-readiness/database-endpoint@1.0.0"
)
ACTION_PACKAGE = (
    "pkg:attestify/acme.example/verification-action/"
    "release-readiness/database-connection@1.0.0"
)
L1_PACKAGE = L0_PACKAGE.replace("@1.0.0", "@1.1.0")
ACTIVITY_PACKAGE = (
    "pkg:attestify/acme.example/verification-activity/"
    "release-readiness/database-readiness@1.0.0"
)
L2_PACKAGE = (
    "pkg:attestify/acme.example/verification-procedure/"
    "release-readiness/transitive-database-readiness@1.0.0"
)

EXPECTED_MANIFESTS = {
    L0_PACKAGE: "sha256:58d5d09c2a74a8dd9a065fa5723dcdd591dac1b1126eaea924dcd70c6be5462d",
    ACTION_PACKAGE: "sha256:040e5e99ba3c640af33aac02c8cf7ff256a52e7f515cd094389d2885d4668339",
    L1_PACKAGE: "sha256:383f12d9b3687c0e6bf7b80d07eb169759a6525ece6f67a07e3b85ef6bb6cb72",
    ACTIVITY_PACKAGE: "sha256:49d5e635359b162a4336d8af6df1fb120f0359edf6770b22da4dca500ee1f7fd",
    L2_PACKAGE: "sha256:af97479a6f63adb99dbff757a5e7f476eb4a3960646b0512d00f2eab9c42f8a1",
}

SOURCE_FOR_PACKAGE = {
    L0_PACKAGE: "l0-procedure",
    ACTION_PACKAGE: "l1-action",
    L1_PACKAGE: "l1-procedure",
    ACTIVITY_PACKAGE: "l2-activity",
    L2_PACKAGE: "l2-procedure",
}


def canonical(value: object) -> bytes:
    return json.dumps(
        value, allow_nan=False, sort_keys=True, separators=(",", ":")
    ).encode()


def digest(data: bytes) -> str:
    return "sha256:" + hashlib.sha256(data).hexdigest()


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


def schema_for(definitions: dict, selector: str) -> dict:
    return {
        "$schema": definitions["$schema"],
        "$defs": definitions["$defs"],
        "$ref": selector,
    }


class Qualification:
    def __init__(self, workspace: Path, root: Path) -> None:
        self.workspace = workspace
        self.root = root
        self.nape = Path(
            os.environ.get(
                "NAPE_I5_BINARY",
                workspace / "applications/nape/target/debug/nape",
            )
        )
        if not self.nape.is_file():
            raise RuntimeError("build applications/nape/target/debug/nape first")

        product_root = workspace / (
            "2-other-workspaces/attestify-design/"
            "attestify-product-specification"
        )
        engine_root = product_root / "05-specs/products/verification-engine/schemas"
        self.source = engine_root / "i0-i5/conformance/source"
        self.golden = engine_root / "i0-i4/conformance/golden/step-4/source"
        self.receipt_schema = json.loads(
            (engine_root / "nape-command-receipt-v2/nape-command-receipt.schema.json")
            .read_bytes()
        )
        self.engine_schema = json.loads(
            (engine_root / "i0-i5/verification-engine-contracts.schema.json")
            .read_bytes()
        )
        self.receipt_validator = jsonschema.Draft202012Validator(
            schema_for(self.receipt_schema, "#/$defs/napeCommandReceiptV2")
        )
        self.report_validator = jsonschema.Draft202012Validator(
            schema_for(self.engine_schema, "#/$defs/verificationReportI5")
        )
        self.relationship_validator = jsonschema.Draft202012Validator(
            schema_for(self.engine_schema, "#/$defs/evidenceSetRelationshipI5")
        )
        self.builds = {package: root / "build" / str(index) for index, package in enumerate(EXPECTED_MANIFESTS)}
        self.environment = self.controlled_evaluator_environment()

    def command(self, arguments: list[str], expected_exit: int = 0) -> dict:
        completed = subprocess.run(
            [str(self.nape), *arguments],
            check=False,
            env=self.environment,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        if completed.returncode != expected_exit:
            raise RuntimeError(
                f"unexpected exit {completed.returncode} for {arguments}: "
                f"{completed.stderr.decode(errors='replace')}"
            )
        if completed.stdout.count(b"\n") != 1 or not completed.stdout.endswith(b"\n"):
            raise RuntimeError(f"command did not emit one line-feed-framed receipt: {arguments}")
        receipt = json.loads(completed.stdout)
        if canonical(receipt) != completed.stdout[:-1]:
            raise RuntimeError(f"receipt is not canonical JSON: {arguments}")
        self.receipt_validator.validate(receipt)
        if receipt["contract"] != "attestify.nape.command-receipt/v2":
            raise RuntimeError("command emitted the wrong Receipt contract")
        return receipt

    def controlled_evaluator_environment(self) -> dict[str, str]:
        executable = self.root / "controlled-nape-eval"
        executable.write_text(
            """#!/usr/bin/env python3
import hashlib, json, pathlib, sys
r=json.load(sys.stdin)
if set(r)!={"contract","workspace_root","evidence","test","evaluations","metadata","limits"} or r["evaluations"]!=[] or "pkg:attestify/" in json.dumps(r):
    raise SystemExit(2)
w=pathlib.Path(r["workspace_root"])
t=(w/r["test"]["file"]).read_bytes()
e=(w/r["evidence"]["file"]).read_bytes()
if "sha256:"+hashlib.sha256(t).hexdigest()!=r["test"]["content_digest"]:
    raise SystemExit(2)
if "sha256:"+hashlib.sha256(e).hexdigest()!=r["evidence"]["argument_digest"]:
    raise SystemExit(2)
test_name=pathlib.Path(r["test"]["file"]).name
if test_name=="configuration-format.py":
    try:
        value=json.loads(e)
    except Exception:
        result={"conclusion":"inconclusive","facts":[{"name":"configuration_format","status":"invalid","value":None,"value_type":None}],"reason":"The supplied configuration is not valid JSON."}
    else:
        valid=isinstance(value,dict)
        result={"conclusion":"true" if valid else "false","facts":[{"name":"configuration_format","status":"found","value":"json-object" if valid else "non-object","value_type":"text"}],"reason":"The supplied configuration was evaluated as a JSON object."}
elif test_name=="database-connection.py":
    expected="postgresql://orders-db.prod.acme.example:5432/orders"
    try:
        value=json.loads(e)
    except Exception:
        result={"conclusion":"inconclusive","facts":[{"name":"database_endpoint","status":"invalid","value":None,"value_type":None}],"reason":"The database endpoint could not be extracted from malformed evidence."}
    else:
        endpoint=value.get("database",{}).get("endpoint") if isinstance(value,dict) and isinstance(value.get("database",{}),dict) else None
        if endpoint is None:
            result={"conclusion":"inconclusive","facts":[{"name":"database_endpoint","status":"not_found","value":None,"value_type":None}],"reason":"The database endpoint fact was not found."}
        else:
            result={"conclusion":"true" if endpoint==expected else "false","facts":[{"name":"database_endpoint","status":"found","value":endpoint,"value_type":"text"}],"reason":"The observed database endpoint was compared with the approved endpoint."}
else:
    raise SystemExit(2)
response={"contract":"attestify.nape-evaluator.action-invocation/v2","diagnostic":None,"disposition":"occurrence-result","execution":{"automatic_retry_count":0,"evaluate_call_count":1,"executed":True,"phase":"evaluate-call","status":"completed"},"result":result,"result_validation":{"contract":"passed","limit":"passed"},"semantic_owner":"test-of-detail"}
print(json.dumps(response,allow_nan=False,sort_keys=True,separators=(",",":")))
""",
            encoding="utf-8",
        )
        executable.chmod(0o700)
        build_record = self.root / "evaluator-build.json"
        record = canonical(
            {
                "contract": "attestify.nape-evaluator.installed-build/v1",
                "contract_projection_digest": "sha256:" + "a" * 64,
                "dependency_set_digest": "sha256:" + "b" * 64,
                "evaluator_contract": "attestify.nape-evaluator.action-invocation/v2",
                "evaluator_release": "2.0.0",
                "implementation_digest": digest(executable.read_bytes()),
                "runner_profile": "attestify-python-test-development-v1",
            }
        )
        build_record.write_bytes(record)
        environment = dict(os.environ)
        environment.update(
            {
                "NAPE_EVALUATOR_V2_EXECUTABLE": str(executable),
                "NAPE_EVALUATOR_V2_BUILD_RECORD": str(build_record),
                "NAPE_EVALUATOR_V2_BUILD_RECORD_SHA256": digest(record),
            }
        )
        return environment

    def build_packages(self) -> None:
        (self.root / "build").mkdir()
        dependencies = {
            L0_PACKAGE: [],
            ACTION_PACKAGE: [],
            L1_PACKAGE: [ACTION_PACKAGE],
            ACTIVITY_PACKAGE: [ACTION_PACKAGE],
            L2_PACKAGE: [ACTIVITY_PACKAGE, ACTION_PACKAGE],
        }
        for package in EXPECTED_MANIFESTS:
            arguments = [
                "package",
                "build",
                "--source",
                str(self.source / SOURCE_FOR_PACKAGE[package]),
                "--package",
                package,
            ]
            for dependency in dependencies[package]:
                arguments.extend(["--dependency-package", str(self.builds[dependency])])
            arguments.extend(["--output", str(self.builds[package])])
            result = self.command(arguments)["result"]
            if result["package"] != package or result["manifestDigest"] != EXPECTED_MANIFESTS[package]:
                raise RuntimeError(f"package identity changed for {package}")

        reverse = self.root / "build-reversed-l2"
        reversed_receipt = self.command(
            [
                "package",
                "build",
                "--source",
                str(self.source / "l2-procedure"),
                "--package",
                L2_PACKAGE,
                "--dependency-package",
                str(self.builds[ACTION_PACKAGE]),
                "--dependency-package",
                str(self.builds[ACTIVITY_PACKAGE]),
                "--output",
                str(reverse),
            ]
        )
        if reversed_receipt["result"]["manifestDigest"] != EXPECTED_MANIFESTS[L2_PACKAGE]:
            raise RuntimeError("dependency argument order changed L2 identity")
        for name in ("manifest.json", "config.json", "package.tar"):
            if (reverse / name).read_bytes() != (self.builds[L2_PACKAGE] / name).read_bytes():
                raise RuntimeError(f"dependency argument order changed {name}")

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

    def publish(self, registry_map: Path, packages: list[str]) -> None:
        for package in packages:
            receipt = self.command(
                [
                    "package",
                    "publish",
                    "--local-package",
                    str(self.builds[package]),
                    "--registry-profile",
                    str(registry_map),
                ]
            )
            if receipt["result"]["disposition"] != "published":
                raise RuntimeError(f"first publication was not new for {package}")
            if receipt["result"]["manifestDigest"] != EXPECTED_MANIFESTS[package]:
                raise RuntimeError(f"publication changed manifest for {package}")
        repeated = self.command(
            [
                "package",
                "publish",
                "--local-package",
                str(self.builds[packages[-1]]),
                "--registry-profile",
                str(registry_map),
            ]
        )
        if repeated["result"]["disposition"] != "already-present":
            raise RuntimeError("same-digest publication is not idempotent")

    def plan(self, registry_map: Path, package: str, edge_count: int) -> None:
        receipt = self.command(
            [
                "package",
                "resolve",
                "--package",
                package,
                "--manifest-digest",
                EXPECTED_MANIFESTS[package],
                "--registry-profile",
                str(registry_map),
                "--plan-only",
            ]
        )
        dependencies = receipt["result"]["dependency"]
        if len(dependencies) != edge_count:
            raise RuntimeError(f"unexpected resolve-plan edge count for {package}")
        if any(set(row) != {"disposition", "from", "location", "manifestDigest", "package", "use"} for row in dependencies):
            raise RuntimeError("resolve-plan Edge projection changed")

    def evidence(self, scenario: str) -> Path:
        root = self.root / "evidence" / scenario
        if scenario == "l2":
            first = root / "infrastructure-readiness/configuration-format/configuration-format.json"
            second = root / "infrastructure-readiness/primary-database/application-configuration.json"
            first.parent.mkdir(parents=True, exist_ok=True)
            second.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(
                self.source.parent / "evidence/l2/configuration-format.json", first
            )
            shutil.copyfile(
                self.source.parent / "evidence/l2/application-configuration.json", second
            )
        else:
            destination = root / "release-readiness/database-connection/application-configuration.json"
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(
                self.golden / "evidence/true/application-configuration.json", destination
            )
        return root

    def verify(
        self, registry_map: Path, package: str, scenario: str, output_name: str
    ) -> tuple[dict, dict, dict]:
        output = self.root / "result" / output_name
        output.parent.mkdir(exist_ok=True)
        receipt = self.command(
            [
                "verify",
                "--package",
                package,
                "--manifest-digest",
                EXPECTED_MANIFESTS[package],
                "--registry-profile",
                str(registry_map),
                "--evidence-root",
                str(self.evidence(scenario)),
                "--subject-file",
                str(self.golden / "subject.json"),
                "--output",
                str(output),
            ]
        )
        report = json.loads((output / "verification-report.json").read_bytes())
        relationship = json.loads((output / "evidence-set-relationship.json").read_bytes())
        self.report_validator.validate(report)
        self.relationship_validator.validate(relationship)
        if receipt["result"]["report"]["summary"] != report["summary"]:
            raise RuntimeError("Receipt summary does not equal committed Report")
        expected_actions = 2 if scenario == "l2" else 1
        if report["summary"]["action_count"] != expected_actions:
            raise RuntimeError("Report action count changed")
        if report["summary"]["actions_completed"] != expected_actions:
            raise RuntimeError("controlled lifecycle did not complete every occurrence")
        if len(relationship["occurrence"]) != expected_actions:
            raise RuntimeError("Evidence Set occurrence count changed")
        payload_files = sorted((output / "evidence/sha256").iterdir())
        if len(payload_files) != len(relationship["payload"]):
            raise RuntimeError("stored evidence and relationship payload inventory differ")
        for payload in relationship["payload"]:
            path = output / "evidence/sha256" / payload["content_digest"].removeprefix("sha256:")
            if not path.is_file() or digest(path.read_bytes()) != payload["content_digest"]:
                raise RuntimeError("digest-addressed evidence payload is absent or changed")
        return receipt, report, relationship

    @staticmethod
    def normalized_l2(report: dict, relationship: dict) -> tuple[dict, dict]:
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
    first_port = canonical_port("ATTESTIFY_I5_REGISTRY_PORT_ONE", "51235")
    second_port = canonical_port("ATTESTIFY_I5_REGISTRY_PORT_TWO", "51236")
    if first_port == second_port:
        raise RuntimeError("I5 qualification requires two distinct registry ports")

    Qualification.require_clean_registry(first_port)
    Qualification.require_clean_registry(second_port)
    with tempfile.TemporaryDirectory(prefix="attestify-i5-lifecycle-") as temporary:
        qualification = Qualification(workspace, Path(temporary))
        qualification.build_packages()
        first_map = qualification.registry_map(first_port)
        second_map = qualification.registry_map(second_port)

        qualification.publish(first_map, list(EXPECTED_MANIFESTS))
        qualification.plan(first_map, L0_PACKAGE, 0)
        qualification.plan(first_map, L1_PACKAGE, 1)
        qualification.plan(first_map, L2_PACKAGE, 2)
        qualification.verify(first_map, L0_PACKAGE, "l0", "first-l0")
        qualification.verify(first_map, L1_PACKAGE, "l1", "first-l1")
        _, first_report, first_relationship = qualification.verify(
            first_map, L2_PACKAGE, "l2", "first-l2"
        )

        qualification.publish(
            second_map, [ACTION_PACKAGE, ACTIVITY_PACKAGE, L2_PACKAGE]
        )
        qualification.plan(second_map, L2_PACKAGE, 2)
        _, second_report, second_relationship = qualification.verify(
            second_map, L2_PACKAGE, "l2", "second-l2"
        )
        if qualification.normalized_l2(first_report, first_relationship) != qualification.normalized_l2(second_report, second_relationship):
            raise RuntimeError("L2 semantic observations differ across clean registries")

        result = {
            "contract": "attestify.nape.i5-full-lifecycle-qualification/v1",
            "manifestDigests": EXPECTED_MANIFESTS,
            "proofs": {
                "firstRegistry": ["L0", "L1", "L2"],
                "secondRegistry": ["L2"],
                "receiptContract": "attestify.nape.command-receipt/v2",
                "semanticEquality": "passed",
            },
            "status": "passed",
        }
        print(canonical(result).decode())


if __name__ == "__main__":
    main()
