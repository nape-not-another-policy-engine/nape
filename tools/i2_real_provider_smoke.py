#!/usr/bin/env python3
"""Execute I2 against the real NAPE Evaluator V2 provider.

The tool creates all transient inputs outside the repository, executes the
authoritative A0 package and GF-01-through-GF-04/GF-06 evidence cases, and
compares two clean true runs after removing only invocation-generated values.
GF-05 remains in the Rust boundary suite because its controlled invalid return
is an Evaluator-response mutation rather than an authored A0 Test behavior.
"""

from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile


PACKAGE = (
    "pkg:attestify/acme.example/verification-procedure/"
    "release-readiness/database-endpoint@1.0.0"
)
IMPLEMENTATION = "sha256:78c94caaf4db0eb6cb70ddf428e13c314e8d4a9b5713c695bdaebaff4e37ae02"
CONTRACT_PROJECTION = "sha256:6ac80bec8e158d38f321e4501d6cb749b39e518c823efcc45c86c2e4b7d2b20d"
DEPENDENCY_SET = "sha256:eccdf4cdc565a952da21880f1b9f16043b9d74b91eb64cbc9e75888d691275bb"


def canonical(value: object) -> bytes:
    return json.dumps(
        value,
        ensure_ascii=False,
        allow_nan=False,
        sort_keys=True,
        separators=(",", ":"),
    ).encode("utf-8")


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


def normalize(report: dict, relationship: dict) -> tuple[dict, dict]:
    report = json.loads(json.dumps(report))
    relationship = json.loads(json.dumps(relationship))
    report["invocation"]["id"] = "<invocation-id>"
    report["metadata"]["id"] = "<report-id>"
    report["metadata"]["generated_at"] = "<generated-at>"
    relationship["report"]["metadata"]["id"] = "<report-id>"
    return report, relationship


def main() -> None:
    workspace = Path(
        os.environ.get(
            "ATTESTIFY_WORKSPACE_ROOT", str(Path(__file__).resolve().parents[3])
        )
    )
    nape = Path(
        os.environ.get(
            "NAPE_I2_BINARY",
            str(workspace / "applications/nape/target/debug/nape"),
        )
    )
    evaluator = workspace / "applications/nape-evaluator"
    golden = (
        workspace
        / "2-other-workspaces/attestify-design/attestify-product-specification"
        / "05-specs/products/verification-engine/schemas/i0-i4/conformance/golden/step-4"
    )
    if not nape.is_file():
        raise RuntimeError("build applications/nape target/debug/nape before running this proof")

    with tempfile.TemporaryDirectory(prefix="attestify-i2-real-provider-") as temporary:
        root = Path(temporary)
        package = root / "package"
        built = receipt(
            run(
                [
                    str(nape),
                    "package",
                    "build",
                    "--source",
                    str(golden / "source/package/a0-procedure"),
                    "--package",
                    PACKAGE,
                    "--output",
                    str(package),
                ]
            ),
            0,
        )
        if built["result"]["manifestDigest"] != "sha256:58d5d09c2a74a8dd9a065fa5723dcdd591dac1b1126eaea924dcd70c6be5462d":
            raise RuntimeError("A0 manifest identity changed")

        configured_wrapper = os.environ.get("NAPE_I2_EVALUATOR")
        if configured_wrapper is None:
            wrapper = root / "nape-eval"
            wrapper.write_text(
                "#!/bin/sh\n"
                f"export PYTHONPATH='{evaluator / 'src'}'\n"
                f"exec python3 '{evaluator / 'main.py'}' \"$@\"\n",
                encoding="utf-8",
            )
            wrapper.chmod(0o700)
        else:
            wrapper = Path(configured_wrapper)
        build_record = root / "evaluator-build.json"
        record_bytes = canonical(
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
        build_record.write_bytes(record_bytes)
        environment = dict(os.environ)
        environment.update(
            {
                "NAPE_EVALUATOR_V2_EXECUTABLE": str(wrapper),
                "NAPE_EVALUATOR_V2_BUILD_RECORD": str(build_record),
                "NAPE_EVALUATOR_V2_BUILD_RECORD_SHA256": digest(record_bytes),
            }
        )
        subject = golden / "source/subject.json"
        exact_profile_runtime = (
            sys.implementation.name == "cpython"
            and sys.version_info[:3] == (3, 11, 6)
            and sys.platform.startswith("linux")
        )
        scenarios = (
            (
                ("true", "true"),
                ("true-repeat", "true"),
                ("false", "false"),
                ("missing", "inconclusive"),
                ("malformed", "inconclusive"),
            )
            if exact_profile_runtime
            else (("true", "inconclusive"),)
        )
        observations: dict[str, tuple[dict, dict]] = {}
        for scenario, conclusion in scenarios:
            evidence_scenario = "true" if scenario == "true-repeat" else scenario
            evidence_root = root / f"evidence-{scenario}"
            evidence_path = (
                evidence_root
                / "release-readiness/database-connection/application-configuration.json"
            )
            evidence_path.parent.mkdir(parents=True)
            shutil.copyfile(
                golden
                / f"source/evidence/{evidence_scenario}/application-configuration.json",
                evidence_path,
            )
            output = root / f"result-{scenario}"
            completed = receipt(
                run(
                    [
                        str(nape),
                        "verify",
                        "--local-package",
                        str(package),
                        "--evidence-root",
                        str(evidence_root),
                        "--subject-file",
                        str(subject),
                        "--output",
                        str(output),
                    ],
                    environment,
                ),
                0,
            )
            if completed["result"]["report"]["conclusion"] != conclusion:
                observed_report = json.loads(
                    (output / "verification-report.json").read_bytes()
                )
                raise RuntimeError(
                    f"{scenario} conclusion did not match: "
                    f"{json.dumps(observed_report, sort_keys=True)}"
                )
            report = json.loads((output / "verification-report.json").read_bytes())
            relationship = json.loads(
                (output / "evidence-set-relationship.json").read_bytes()
            )
            if report["activity"][0]["action"][0]["conclusion"] != conclusion:
                raise RuntimeError(f"{scenario} Report conclusion did not match")
            observations[scenario] = normalize(report, relationship)

        if exact_profile_runtime:
            if observations["true"] != observations["true-repeat"]:
                raise RuntimeError(
                    "two clean true executions differed outside generated values"
                )
        else:
            action = observations["true"][0]["activity"][0]["action"][0]
            diagnostics = observations["true"][0]["processing_detail"]["diagnostics"]
            if (
                action["execution"]["status"] != "blocked"
                or action["execution"]["executed"] is not False
                or len(diagnostics) != 1
                or diagnostics[0]["code"] != "runner_activation_failed"
            ):
                raise RuntimeError("non-profile host did not produce the bounded blocked result")

        absent_root = root / "evidence-absent"
        absent_root.mkdir()
        absent_output = root / "result-absent"
        terminal = receipt(
            run(
                [
                    str(nape),
                    "verify",
                    "--local-package",
                    str(package),
                    "--evidence-root",
                    str(absent_root),
                    "--subject-file",
                    str(subject),
                    "--output",
                    str(absent_output),
                ],
                environment,
            ),
            1,
        )
        if terminal["diagnostic"]["code"] != "evidence_association_invalid":
            raise RuntimeError("GF-06 diagnostic changed")
        if absent_output.exists():
            raise RuntimeError("GF-06 promoted a result")

        print(
            canonical(
                {
                    "contract": "attestify.verification-engine.i2-real-provider-smoke/v1",
                    "clean_true_runs_equal": True if exact_profile_runtime else "not-run",
                    "gf01_through_gf04": (
                        "passed" if exact_profile_runtime else "exact-profile-runtime-required"
                    ),
                    "gf06": "passed",
                    "manifest_digest": built["result"]["manifestDigest"],
                    "network_or_oci_used": False,
                    "provider": "nape-evaluator-v2",
                    "runtime_observation": (
                        "exact-profile-executed"
                        if exact_profile_runtime
                        else "bounded-runner-activation-blocked"
                    ),
                }
            ).decode("utf-8")
        )


if __name__ == "__main__":
    main()
