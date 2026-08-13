#!/usr/bin/env python3
"""Exercise the bounded I4 one-locked-Action milestone against localhost Distribution."""

from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import urllib.parse
import urllib.request

try:
    import jsonschema
except ImportError:  # The runtime proof remains usable without the optional validator.
    jsonschema = None


PROCEDURE_PACKAGE = (
    "pkg:attestify/acme.example/verification-procedure/"
    "release-readiness/database-endpoint@1.1.0"
)
ACTION_PACKAGE = (
    "pkg:attestify/acme.example/verification-action/"
    "release-readiness/database-connection@1.0.0"
)
PROCEDURE_REPOSITORY = (
    "attestify/acme.example/verification-procedure/"
    "release-readiness/database-endpoint"
)
ACTION_REPOSITORY = (
    "attestify/acme.example/verification-action/"
    "release-readiness/database-connection"
)
PROCEDURE_DIGEST = "sha256:383f12d9b3687c0e6bf7b80d07eb169759a6525ece6f67a07e3b85ef6bb6cb72"
ACTION_DIGEST = "sha256:040e5e99ba3c640af33aac02c8cf7ff256a52e7f515cd094389d2885d4668339"
ACTION_TEST_DIGEST = "sha256:ed7fc9dd94004c451364cf79cfc07db93a03e9786040b9da34fcbb01a7c38bcc"
REGISTRY_PORT_TEXT = os.environ.get("ATTESTIFY_I4_REGISTRY_PORT", "5001")
if (
    not REGISTRY_PORT_TEXT.isascii()
    or not REGISTRY_PORT_TEXT.isdecimal()
    or REGISTRY_PORT_TEXT.startswith("0")
    or not 1 <= int(REGISTRY_PORT_TEXT) <= 65535
):
    raise RuntimeError("ATTESTIFY_I4_REGISTRY_PORT must be a canonical TCP port")
REGISTRY_PORT = int(REGISTRY_PORT_TEXT)
ORIGIN = f"http://localhost:{REGISTRY_PORT}"
IMPLEMENTATION = "sha256:78c94caaf4db0eb6cb70ddf428e13c314e8d4a9b5713c695bdaebaff4e37ae02"
CONTRACT_PROJECTION = "sha256:6ac80bec8e158d38f321e4501d6cb749b39e518c823efcc45c86c2e4b7d2b20d"
DEPENDENCY_SET = "sha256:eccdf4cdc565a952da21880f1b9f16043b9d74b91eb64cbc9e75888d691275bb"


def canonical(value: object) -> bytes:
    return json.dumps(value, allow_nan=False, sort_keys=True, separators=(",", ":")).encode()


def digest(data: bytes) -> str:
    return "sha256:" + hashlib.sha256(data).hexdigest()


def run(command: list[str], environment: dict[str, str] | None = None) -> subprocess.CompletedProcess[bytes]:
    return subprocess.run(
        command,
        env=environment,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )


def receipt(completed: subprocess.CompletedProcess[bytes], expected_exit: int) -> dict:
    if completed.returncode != expected_exit:
        raise RuntimeError(
            f"unexpected exit {completed.returncode}: {completed.stderr.decode(errors='replace')}"
        )
    if completed.stdout.count(b"\n") != 1 or not completed.stdout.endswith(b"\n"):
        raise RuntimeError("command did not emit one line-feed-framed receipt")
    value = json.loads(completed.stdout)
    if canonical(value) != completed.stdout[:-1]:
        raise RuntimeError("command receipt is not canonical JCS")
    return value


def request(
    method: str,
    path: str,
    data: bytes = b"",
    content_type: str | None = None,
) -> urllib.response.addinfourl:
    headers = {}
    if content_type is not None:
        headers["Content-Type"] = content_type
    return urllib.request.urlopen(
        urllib.request.Request(ORIGIN + path, data=data, headers=headers, method=method),
        timeout=10,
    )


def seed_blob(repository: str, blob: bytes) -> None:
    expected = digest(blob)
    begin = request("POST", f"/v2/{repository}/blobs/uploads/")
    location = begin.headers["Location"]
    parsed = urllib.parse.urlsplit(location)
    query = urllib.parse.parse_qsl(parsed.query, keep_blank_values=True)
    query.append(("digest", expected))
    completed = request(
        "PUT",
        parsed.path + "?" + urllib.parse.urlencode(query),
        blob,
        "application/octet-stream",
    )
    if completed.status != 201 or completed.headers.get("Docker-Content-Digest") != expected:
        raise RuntimeError("registry did not commit the exact blob")


def seed_package(package: Path, repository: str, tag: str, expected_digest: str) -> None:
    manifest = (package / "manifest.json").read_bytes()
    config = (package / "config.json").read_bytes()
    archive = (package / "package.tar").read_bytes()
    parsed = json.loads(manifest)
    if digest(manifest) != expected_digest:
        raise RuntimeError("frozen package manifest identity changed")
    if digest(config) != parsed["config"]["digest"] or digest(archive) != parsed["layers"][0]["digest"]:
        raise RuntimeError("frozen package descriptors do not equal their bytes")
    seed_blob(repository, config)
    seed_blob(repository, archive)
    for reference in (expected_digest, tag):
        response = request(
            "PUT",
            f"/v2/{repository}/manifests/{reference}",
            manifest,
            "application/vnd.oci.image.manifest.v1+json",
        )
        if response.status != 201 or response.headers.get("Docker-Content-Digest") != expected_digest:
            raise RuntimeError("registry did not commit the exact manifest")


def evaluator_environment(root: Path, evaluator: Path) -> dict[str, str]:
    wrapper = root / "nape-eval"
    wrapper.write_text(
        "#!/bin/sh\n"
        + f"export PYTHONPATH='{evaluator / 'src'}'\n"
        + f"exec python3 '{evaluator / 'main.py'}' \"$@\"\n",
        encoding="utf-8",
    )
    wrapper.chmod(0o700)
    build_record = root / "evaluator-build.json"
    record = canonical(
        {
            "contract": "attestify.nape-evaluator.installed-build/v1",
            "contract_projection_digest": CONTRACT_PROJECTION,
            "dependency_set_digest": DEPENDENCY_SET,
            "evaluator_contract": "attestify.nape-evaluator.action-invocation/v2",
            "evaluator_release": "2.0.0",
            "implementation_digest": IMPLEMENTATION,
            "runner_profile": "attestify-python-test-development-v1",
        }
    )
    build_record.write_bytes(record)
    environment = dict(os.environ)
    environment.update(
        {
            "NAPE_EVALUATOR_V2_EXECUTABLE": str(wrapper),
            "NAPE_EVALUATOR_V2_BUILD_RECORD": str(build_record),
            "NAPE_EVALUATOR_V2_BUILD_RECORD_SHA256": digest(record),
        }
    )
    return environment


def controlled_evaluator_environment(root: Path, base: dict[str, str]) -> dict[str, str]:
    """Provide exact I1 outcomes while preserving the production consumer boundary."""
    executable = root / "controlled-nape-eval"
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
response={"contract":"attestify.nape-evaluator.action-invocation/v2","diagnostic":None,"disposition":"occurrence-result","execution":{"automatic_retry_count":0,"evaluate_call_count":1,"executed":True,"phase":"evaluate-call","status":"completed"},"result":result,"result_validation":{"contract":"passed","limit":"passed"},"semantic_owner":"test-of-detail"}
print(json.dumps(response,allow_nan=False,sort_keys=True,separators=(",",":")))
""",
        encoding="utf-8",
    )
    executable.chmod(0o700)
    environment = dict(base)
    environment["NAPE_EVALUATOR_V2_EXECUTABLE"] = str(executable)
    return environment


def normalized_result(output: Path) -> tuple[dict, dict, bytes]:
    report = json.loads((output / "verification-report.json").read_bytes())
    relationship = json.loads((output / "evidence-set-relationship.json").read_bytes())
    evidence = next((output / "evidence/sha256").iterdir()).read_bytes()
    report["invocation"]["id"] = "<invocation-id>"
    report["metadata"]["id"] = "<report-id>"
    report["metadata"]["generated_at"] = "<generated-at>"
    relationship["report"]["metadata"]["id"] = "<report-id>"
    return report, relationship, evidence


def validate_product_output(schema: dict, report: dict, relationship: dict) -> None:
    if jsonschema is None:
        return
    base = {"$schema": schema["$schema"], "$defs": schema["$defs"]}
    report_schema = dict(base)
    report_schema["$ref"] = "#/$defs/verificationReportI5"
    relationship_schema = dict(base)
    relationship_schema["$ref"] = "#/$defs/evidenceSetRelationshipI5"
    jsonschema.Draft202012Validator(report_schema).validate(report)
    jsonschema.Draft202012Validator(relationship_schema).validate(relationship)


def main() -> None:
    workspace = Path(os.environ.get("ATTESTIFY_WORKSPACE_ROOT", Path(__file__).resolve().parents[3]))
    nape = Path(os.environ.get("NAPE_I4_BINARY", workspace / "applications/nape/target/debug/nape"))
    evaluator = workspace / "applications/nape-evaluator"
    golden = workspace / (
        "2-other-workspaces/attestify-design/attestify-product-specification/"
        "05-specs/products/verification-engine/schemas/i0-i4/conformance/golden/step-4"
    )
    if not nape.is_file():
        raise RuntimeError("build applications/nape target/debug/nape before running this proof")
    product_schema = json.loads(
        (golden.parents[3] / "i0-i5/verification-engine-contracts.schema.json").read_bytes()
    )

    with tempfile.TemporaryDirectory(prefix="attestify-i4-real-registry-") as temporary:
        root = Path(temporary)
        registry_map = root / "registry-map.yaml"
        registry_map.write_text(
            "profileVersion: attestify-oci-registry-map/1\n"
            "publishers:\n  acme.example:\n    scheme: http\n"
            f"    registry: localhost:{REGISTRY_PORT}\n"
            "    repositoryPrefix: attestify\n",
            encoding="utf-8",
        )
        procedure = golden / "generated/package/a3-procedure"
        action = golden / "generated/package/a3-action"
        seed_package(procedure, PROCEDURE_REPOSITORY, "1.1.0", PROCEDURE_DIGEST)

        planned = receipt(
            run(
                [
                    str(nape),
                    "package",
                    "resolve",
                    "--package",
                    PROCEDURE_PACKAGE,
                    "--manifest-digest",
                    PROCEDURE_DIGEST,
                    "--registry-profile",
                    str(registry_map),
                    "--plan-only",
                ]
            ),
            0,
        )
        dependency = planned["result"]["dependency"]
        if len(dependency) != 1 or dependency[0] != {
            "disposition": "planned-not-acquired",
            "from": PROCEDURE_PACKAGE,
            "location": {
                "profileVersion": "attestify-oci-registry-map/1",
                "publisher": "acme.example",
                "reference": ACTION_DIGEST,
                "referenceClass": "digest",
                "registry": f"localhost:{REGISTRY_PORT}",
                "repository": ACTION_REPOSITORY,
                "scheme": "http",
            },
            "manifestDigest": ACTION_DIGEST,
            "package": ACTION_PACKAGE,
            "use": "release-readiness.database-connection",
        }:
            raise RuntimeError(
                "resolve-plan dependency projection changed: "
                + canonical(dependency).decode()
            )

        real_environment = evaluator_environment(root, evaluator)
        evidence_root = root / "evidence"
        evidence_path = evidence_root / (
            "release-readiness/database-connection/application-configuration.json"
        )
        evidence_path.parent.mkdir(parents=True)
        shutil.copyfile(golden / "source/evidence/true/application-configuration.json", evidence_path)
        subject = golden / "source/subject.json"
        unavailable_output = root / "unavailable-output"
        unavailable = receipt(
            run(
                [
                    str(nape), "verify", "--package", PROCEDURE_PACKAGE,
                    "--manifest-digest", PROCEDURE_DIGEST,
                    "--registry-profile", str(registry_map),
                    "--evidence-root", str(evidence_root),
                    "--subject-file", str(subject), "--output", str(unavailable_output),
                ],
                real_environment,
            ),
            1,
        )
        if unavailable["diagnostic"]["code"] != "dependency_unavailable" or unavailable_output.exists():
            raise RuntimeError("missing Action did not fail atomically at dependency acquisition")

        seed_package(action, ACTION_REPOSITORY, "1.0.0", ACTION_DIGEST)
        environment = controlled_evaluator_environment(root, real_environment)
        normalized = None
        conclusions: dict[str, str] = {}
        scenarios = (
            ("true", "true"),
            ("false", "false"),
            ("missing", "inconclusive"),
            ("malformed", "inconclusive"),
        )
        for scenario, expected in scenarios:
            shutil.copyfile(
                golden / f"source/evidence/{scenario}/application-configuration.json",
                evidence_path,
            )
            output = root / f"result-{scenario}"
            completed = receipt(
                run(
                    [
                        str(nape), "verify", "--package", PROCEDURE_PACKAGE,
                        "--manifest-digest", PROCEDURE_DIGEST,
                        "--registry-profile", str(registry_map),
                        "--evidence-root", str(evidence_root),
                        "--subject-file", str(subject), "--output", str(output),
                    ],
                    environment,
                ),
                0,
            )
            report = json.loads((output / "verification-report.json").read_bytes())
            relationship = json.loads(
                (output / "evidence-set-relationship.json").read_bytes()
            )
            validate_product_output(product_schema, report, relationship)
            action_row = report["activity"][0]["action"][0]
            observed = action_row["conclusion"]
            if observed != expected:
                raise RuntimeError(
                    f"{scenario} produced conclusion {observed!r}, expected {expected!r}; "
                    + canonical(
                        {
                            "diagnostics": report["processing_detail"]["diagnostics"],
                            "execution": action_row["execution"],
                            "reason": action_row["reason"],
                        }
                    ).decode()
                )
            if (
                report["procedure"]["package"] != PROCEDURE_PACKAGE
                or report["procedure"]["manifestDigest"] != PROCEDURE_DIGEST
                or action_row["definition_origin"]["package"] != ACTION_PACKAGE
                or action_row["definition_origin"]["manifestDigest"] != ACTION_DIGEST
                or action_row["definition_origin"]["selector"]
                != "release-readiness.database-connection"
                or action_row["test"]["owner"]["package"] != ACTION_PACKAGE
                or action_row["test"]["resolved_file"]["path"]
                != "action/database-connection/database-connection.py"
                or action_row["test"]["resolved_file"]["content_digest"]
                != ACTION_TEST_DIGEST
                or action_row["test"]["resolved_file"]["byte_count"] != 4663
                or relationship["occurrence"][0]["action"]
                != "release-readiness.database-connection"
                or relationship["occurrence"][0]["content_digest"]
                != relationship["payload"][0]["content_digest"]
            ):
                raise RuntimeError("I4 Report origin or ownership projection changed")
            if scenario == "true":
                normalized = normalized_result(output)
            conclusions[scenario] = expected

        shutil.copyfile(golden / "source/evidence/true/application-configuration.json", evidence_path)
        repeat_output = root / "result-true-repeat"
        receipt(
            run(
                [
                    str(nape), "verify", "--package", PROCEDURE_PACKAGE,
                    "--manifest-digest", PROCEDURE_DIGEST,
                    "--registry-profile", str(registry_map),
                    "--evidence-root", str(evidence_root),
                    "--subject-file", str(subject), "--output", str(repeat_output),
                ],
                environment,
            ),
            0,
        )
        if normalized_result(repeat_output) != normalized:
            raise RuntimeError("two clean I4 executions differ in deterministic fields")

        exact_profile_runtime = (
            sys.implementation.name == "cpython"
            and sys.version_info[:3] == (3, 11, 6)
            and sys.platform.startswith("linux")
        )
        real_output = root / "result-real-provider"
        real = receipt(
            run(
                [
                    str(nape), "verify", "--package", PROCEDURE_PACKAGE,
                    "--manifest-digest", PROCEDURE_DIGEST,
                    "--registry-profile", str(registry_map),
                    "--evidence-root", str(evidence_root),
                    "--subject-file", str(subject), "--output", str(real_output),
                ],
                real_environment,
            ),
            0,
        )
        real_report = json.loads((real_output / "verification-report.json").read_bytes())
        real_relationship = json.loads(
            (real_output / "evidence-set-relationship.json").read_bytes()
        )
        validate_product_output(product_schema, real_report, real_relationship)
        real_action = real_report["activity"][0]["action"][0]
        if exact_profile_runtime:
            if real_action["conclusion"] != "true":
                raise RuntimeError("exact profile did not execute the real Test successfully")
        else:
            diagnostics = real_report["processing_detail"]["diagnostics"]
            if (
                real_action["conclusion"] != "inconclusive"
                or real_action["execution"]["status"] != "blocked"
                or real_action["execution"]["executed"] is not False
                or len(diagnostics) != 1
                or diagnostics[0]["code"] != "runner_activation_failed"
            ):
                raise RuntimeError("non-profile host did not produce the bounded blocked result")

        print(
            canonical(
                {
                    "actionManifestDigest": ACTION_DIGEST,
                    "conclusions": conclusions,
                    "deterministicRepeat": "passed",
                    "exactProfileExecution": "passed"
                    if exact_profile_runtime
                    else "exact-profile-runtime-required",
                    "missingActionAtomicity": "passed",
                    "procedureManifestDigest": PROCEDURE_DIGEST,
                    "resolvePlan": "passed",
                    "runtimeObservation": "exact-profile-executed"
                    if exact_profile_runtime
                    else "bounded-runner-activation-blocked",
                    "schemaValidation": "passed"
                    if jsonschema is not None
                    else "optional-validator-not-installed",
                }
            ).decode()
        )


if __name__ == "__main__":
    main()
