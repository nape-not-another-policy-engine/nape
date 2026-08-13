#!/usr/bin/env python3
"""Verify one standalone Step-7 read-only contract projection."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import stat
import sys
from pathlib import Path, PurePosixPath
from typing import Any


MANIFEST_CONTRACT = (
    "attestify.nape-evaluator.repository-contract-projection/v1"
)
PIN_CONTRACT = "attestify.nape-evaluator.repository-contract-projection-pin/v1"
SOURCE_CONTRACT = "attestify.nape-evaluator.action-invocation/v2"
ROLES = {"nape-orchestrator-consumer", "nape-evaluator-provider"}
MANIFEST_FIELDS = {
    "conformance_payload",
    "conformance_payload_count",
    "conformance_source_set_sha256",
    "contract",
    "generated_control",
    "generated_control_count",
    "generated_read_only",
    "historical_bridge",
    "projection_role",
    "runtime_payload",
    "runtime_payload_count",
    "runtime_source_set_sha256",
    "source_authority",
    "source_contract",
    "source_payload_count",
    "source_set_sha256",
}
PIN_FIELDS = {
    "contract",
    "manifest_bytes",
    "manifest_sha256",
    "projection_role",
    "source_set_sha256",
}
RECORD_FIELDS = {"bytes", "projection_path", "sha256", "source_path"}
HISTORICAL_BRIDGE_FIELDS = {
    "artifact_step_1_transition",
    "current_step7_source_set",
    "historical_step6_source_set",
    "source_set_transition",
}
IDENTITY_FIELDS = {"bytes", "path", "sha256"}
TRANSITION_FIELDS = IDENTITY_FIELDS | {"changed_payload_count"}
EXPECTED_RUNTIME_PAYLOAD_COUNT = 5
EXPECTED_CONFORMANCE_PAYLOAD_COUNT = 48
EXPECTED_GENERATED_CONTROL_COUNT = 2
EXPECTED_SOURCE_PAYLOAD_COUNT = 55
EXPECTED_RUNTIME_SOURCE_PATHS = {
    "05-specs/products/verification-engine/schemas/i0-i4/"
    "verification-engine-contract-index.json",
    "05-specs/products/verification-engine/schemas/i0-i4/"
    "verification-engine-contracts.schema.json",
    "05-specs/products/verification-engine/schemas/"
    "machine-readable-error-registry-v1/error-registry-contract-index.json",
    "05-specs/products/verification-engine/schemas/"
    "machine-readable-error-registry-v1/error-registry.json",
    "05-specs/products/verification-engine/schemas/"
    "machine-readable-error-registry-v1/error-registry.schema.json",
}
EXPECTED_CONTROL_SOURCE_PATHS = {
    "05-specs/products/verification-engine/schemas/i0-i4/conformance/"
    "step-7/artifact-step-2/GENERATED-READ-ONLY.md",
    "scripts/attestify_step7_contract_projection_verifier.py",
}
EXPECTED_CURRENT_SOURCE_SET = (
    "05-specs/products/verification-engine/schemas/i0-i4/conformance/"
    "step-7/artifact-step-2/current-evaluator-contract-source-set.json"
)
EXPECTED_VECTOR_MANIFEST = (
    "05-specs/products/verification-engine/schemas/i0-i4/conformance/"
    "vector-manifest.json"
)
EXPECTED_VECTOR_PREFIX = (
    "05-specs/products/verification-engine/schemas/i0-i4/conformance/"
    "vectors/"
)
SHA256_PREFIX = "sha256:"
OUTPUT_LIMIT = 65_536


class ProjectionError(ValueError):
    """A closed standalone projection check failed."""


def reject(code: str) -> None:
    raise ProjectionError(code)


def sha256_bytes(raw: bytes) -> str:
    return SHA256_PREFIX + hashlib.sha256(raw).hexdigest()


def _no_duplicate_pairs(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise ValueError("duplicate JSON member")
        result[key] = value
    return result


def strict_json(raw: bytes, code: str) -> Any:
    try:
        return json.loads(
            raw.decode("utf-8"),
            object_pairs_hook=_no_duplicate_pairs,
            parse_constant=lambda value: (_ for _ in ()).throw(
                ValueError(f"invalid JSON constant: {value}")
            ),
        )
    except (UnicodeDecodeError, ValueError, json.JSONDecodeError) as error:
        raise ProjectionError(code) from error


def valid_sha256(value: Any) -> bool:
    return (
        isinstance(value, str)
        and len(value) == 71
        and value.startswith(SHA256_PREFIX)
        and all(character in "0123456789abcdef" for character in value[7:])
    )


def canonical_relative(value: Any, code: str) -> PurePosixPath:
    if not isinstance(value, str) or not value or "\\" in value:
        reject(code)
    path = PurePosixPath(value)
    if (
        path.is_absolute()
        or path.as_posix() != value
        or any(part in {"", ".", ".."} for part in path.parts)
    ):
        reject(code)
    return path


def read_regular(root: Path, relative: PurePosixPath, code: str) -> bytes:
    current = root
    try:
        for index, part in enumerate(relative.parts):
            current = current / part
            metadata = current.lstat()
            if stat.S_ISLNK(metadata.st_mode):
                reject("projection-symbolic-link")
            if index < len(relative.parts) - 1:
                if not stat.S_ISDIR(metadata.st_mode):
                    reject(code)
            elif not stat.S_ISREG(metadata.st_mode):
                reject(code)
        resolved = current.resolve(strict=True)
        resolved.relative_to(root)
        return current.read_bytes()
    except ProjectionError:
        raise
    except (OSError, ValueError) as error:
        raise ProjectionError(code) from error


def aggregate_identity(records: list[tuple[str, bytes]]) -> str:
    digest = hashlib.sha256()
    for path, raw in sorted(records):
        digest.update(path.encode("utf-8"))
        digest.update(b"\0")
        digest.update(len(raw).to_bytes(8, "big"))
        digest.update(raw)
    return SHA256_PREFIX + digest.hexdigest()


def _validate_bridge_identity(value: Any, fields: set[str]) -> None:
    if not isinstance(value, dict) or set(value) != fields:
        reject("projection-historical-bridge")
    canonical_relative(value["path"], "projection-historical-bridge")
    if (
        not isinstance(value["bytes"], int)
        or isinstance(value["bytes"], bool)
        or value["bytes"] < 1
        or not valid_sha256(value["sha256"])
    ):
        reject("projection-historical-bridge")
    if fields == TRANSITION_FIELDS and (
        not isinstance(value["changed_payload_count"], int)
        or isinstance(value["changed_payload_count"], bool)
        or value["changed_payload_count"] != 17
    ):
        reject("projection-historical-bridge")


def _validate_historical_bridge(value: Any) -> None:
    if not isinstance(value, dict) or set(value) != HISTORICAL_BRIDGE_FIELDS:
        reject("projection-historical-bridge")
    _validate_bridge_identity(
        value["historical_step6_source_set"],
        IDENTITY_FIELDS,
    )
    _validate_bridge_identity(
        value["current_step7_source_set"],
        IDENTITY_FIELDS,
    )
    _validate_bridge_identity(
        value["source_set_transition"],
        TRANSITION_FIELDS,
    )
    _validate_bridge_identity(
        value["artifact_step_1_transition"],
        IDENTITY_FIELDS,
    )


def _validate_record_list(
    root: Path,
    value: Any,
    prefix: str,
    code: str,
) -> tuple[list[tuple[str, bytes]], set[str]]:
    if not isinstance(value, list):
        reject(code)
    source_records: list[tuple[str, bytes]] = []
    local_paths: set[str] = set()
    source_paths: list[str] = []
    for record in value:
        if not isinstance(record, dict) or set(record) != RECORD_FIELDS:
            reject(code)
        source = canonical_relative(record["source_path"], code).as_posix()
        local = canonical_relative(record["projection_path"], code).as_posix()
        if (
            not local.startswith(prefix)
            or local in local_paths
            or not isinstance(record["bytes"], int)
            or isinstance(record["bytes"], bool)
            or record["bytes"] < 1
            or not valid_sha256(record["sha256"])
        ):
            reject(code)
        raw = read_regular(root, PurePosixPath(local), code)
        if len(raw) != record["bytes"] or sha256_bytes(raw) != record["sha256"]:
            reject(code)
        local_paths.add(local)
        source_paths.append(source)
        source_records.append((source, raw))
    if source_paths != sorted(source_paths) or len(source_paths) != len(
        set(source_paths)
    ):
        reject(code)
    return source_records, local_paths


def _actual_file_set(root: Path) -> set[str]:
    files: set[str] = set()
    for path in root.rglob("*"):
        try:
            metadata = path.lstat()
        except OSError as error:
            raise ProjectionError("projection-file-inventory") from error
        if stat.S_ISLNK(metadata.st_mode):
            reject("projection-symbolic-link")
        if stat.S_ISREG(metadata.st_mode):
            files.add(path.relative_to(root).as_posix())
        elif not stat.S_ISDIR(metadata.st_mode):
            reject("projection-file-kind")
    return files


def verify_projection(root: Path) -> dict[str, Any]:
    if not root.is_absolute():
        reject("projection-root-not-absolute")
    try:
        metadata = root.lstat()
        resolved = root.resolve(strict=True)
    except OSError as error:
        raise ProjectionError("projection-root-unavailable") from error
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
        reject("projection-root-not-directory")

    manifest_raw = read_regular(
        resolved,
        PurePosixPath("projection-manifest.json"),
        "projection-manifest-file",
    )
    pin_raw = read_regular(
        resolved,
        PurePosixPath("projection-pin.json"),
        "projection-pin-file",
    )
    manifest = strict_json(manifest_raw, "projection-manifest-json")
    pin = strict_json(pin_raw, "projection-pin-json")
    if not isinstance(pin, dict) or set(pin) != PIN_FIELDS:
        reject("projection-pin-contract")
    if (
        pin["contract"] != PIN_CONTRACT
        or pin["projection_role"] not in ROLES
        or not isinstance(pin["manifest_bytes"], int)
        or isinstance(pin["manifest_bytes"], bool)
        or pin["manifest_bytes"] != len(manifest_raw)
        or not valid_sha256(pin["manifest_sha256"])
        or pin["manifest_sha256"] != sha256_bytes(manifest_raw)
        or not valid_sha256(pin["source_set_sha256"])
    ):
        reject("projection-pin-contract")
    if not isinstance(manifest, dict) or set(manifest) != MANIFEST_FIELDS:
        reject("projection-manifest-contract")
    if (
        manifest["contract"] != MANIFEST_CONTRACT
        or manifest["projection_role"] != pin["projection_role"]
        or manifest["projection_role"] not in ROLES
        or manifest["generated_read_only"] is not True
        or manifest["source_authority"] != "attestify-product-specification"
        or manifest["source_contract"] != SOURCE_CONTRACT
        or not valid_sha256(manifest["source_set_sha256"])
        or manifest["source_set_sha256"] != pin["source_set_sha256"]
        or not valid_sha256(manifest["runtime_source_set_sha256"])
        or not valid_sha256(manifest["conformance_source_set_sha256"])
    ):
        reject("projection-manifest-contract")
    _validate_historical_bridge(manifest["historical_bridge"])

    runtime, runtime_paths = _validate_record_list(
        resolved,
        manifest["runtime_payload"],
        "runtime/",
        "projection-runtime-payload",
    )
    conformance, conformance_paths = _validate_record_list(
        resolved,
        manifest["conformance_payload"],
        "conformance/",
        "projection-conformance-payload",
    )
    controls, control_paths = _validate_record_list(
        resolved,
        manifest["generated_control"],
        "",
        "projection-generated-control",
    )
    if control_paths != {
        "GENERATED-READ-ONLY.md",
        "tools/verify_projection.py",
    }:
        reject("projection-generated-control")
    runtime_source_paths = {path for path, _ in runtime}
    conformance_source_paths = {path for path, _ in conformance}
    control_source_paths = {path for path, _ in controls}
    vector_source_paths = {
        path
        for path in conformance_source_paths
        if path.startswith(EXPECTED_VECTOR_PREFIX)
    }
    if (
        runtime_source_paths != EXPECTED_RUNTIME_SOURCE_PATHS
        or control_source_paths != EXPECTED_CONTROL_SOURCE_PATHS
        or EXPECTED_CURRENT_SOURCE_SET not in conformance_source_paths
        or EXPECTED_VECTOR_MANIFEST not in conformance_source_paths
        or len(vector_source_paths) != 46
        or conformance_source_paths
        != vector_source_paths
        | {EXPECTED_CURRENT_SOURCE_SET, EXPECTED_VECTOR_MANIFEST}
    ):
        reject("projection-source-selection")
    all_source_paths = (
        runtime_source_paths | conformance_source_paths | control_source_paths
    )
    if len(all_source_paths) != (
        len(runtime_source_paths)
        + len(conformance_source_paths)
        + len(control_source_paths)
    ):
        reject("projection-source-path-collision")
    all_paths = runtime_paths | conformance_paths | control_paths
    if len(all_paths) != (
        len(runtime_paths) + len(conformance_paths) + len(control_paths)
    ):
        reject("projection-local-path-collision")
    if (
        manifest["runtime_payload_count"] != EXPECTED_RUNTIME_PAYLOAD_COUNT
        or manifest["runtime_payload_count"] != len(runtime)
        or manifest["conformance_payload_count"]
        != EXPECTED_CONFORMANCE_PAYLOAD_COUNT
        or manifest["conformance_payload_count"] != len(conformance)
        or manifest["generated_control_count"]
        != EXPECTED_GENERATED_CONTROL_COUNT
        or manifest["generated_control_count"] != len(controls)
        or manifest["source_payload_count"] != EXPECTED_SOURCE_PAYLOAD_COUNT
        or manifest["source_payload_count"]
        != len(runtime) + len(conformance) + len(controls)
        or manifest["runtime_source_set_sha256"]
        != aggregate_identity(runtime)
        or manifest["conformance_source_set_sha256"]
        != aggregate_identity(conformance)
        or manifest["source_set_sha256"]
        != aggregate_identity(runtime + conformance + controls)
    ):
        reject("projection-source-set-identity")
    expected_files = all_paths | {
        "projection-manifest.json",
        "projection-pin.json",
    }
    if _actual_file_set(resolved) != expected_files:
        reject("projection-file-inventory")

    return {
        "contract": MANIFEST_CONTRACT,
        "projection_role": manifest["projection_role"],
        "manifest_bytes": len(manifest_raw),
        "manifest_sha256": sha256_bytes(manifest_raw),
        "source_payload_count": manifest["source_payload_count"],
        "source_set_sha256": manifest["source_set_sha256"],
        "status": "verified",
    }


def parse_arguments(arguments: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--projection-root",
        type=Path,
        default=Path(__file__).resolve().parents[1],
    )
    return parser.parse_args(arguments)


def main(arguments: list[str] | None = None) -> int:
    options = parse_arguments(sys.argv[1:] if arguments is None else arguments)
    result = verify_projection(options.projection_root)
    encoded = (
        json.dumps(result, separators=(",", ":"), sort_keys=True).encode(
            "utf-8"
        )
        + b"\n"
    )
    if len(encoded) > OUTPUT_LIMIT:
        reject("projection-verifier-output-limit")
    os.write(sys.stdout.fileno(), encoded)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ProjectionError as error:
        print(str(error), file=sys.stderr)
        raise SystemExit(1)
