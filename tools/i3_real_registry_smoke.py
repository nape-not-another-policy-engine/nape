#!/usr/bin/env python3
"""Exercise the bounded I3 root-only OCI milestone against localhost Distribution."""

from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import urllib.error
import urllib.parse
import urllib.request


ROOT_PACKAGE = (
    "pkg:attestify/acme.example/verification-procedure/"
    "release-readiness/database-endpoint@1.0.0"
)
PUBLISHED_PACKAGE = ROOT_PACKAGE.replace("@1.0.0", "@1.0.1")
REPOSITORY = (
    "attestify/acme.example/verification-procedure/"
    "release-readiness/database-endpoint"
)
REGISTRY_PORT_TEXT = os.environ.get("ATTESTIFY_I3_REGISTRY_PORT", "5000")
if (
    not REGISTRY_PORT_TEXT.isascii()
    or not REGISTRY_PORT_TEXT.isdecimal()
    or REGISTRY_PORT_TEXT.startswith("0")
    or not 1 <= int(REGISTRY_PORT_TEXT) <= 65535
):
    raise RuntimeError("ATTESTIFY_I3_REGISTRY_PORT must be a canonical TCP port")
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
    return subprocess.run(command, env=environment, check=False, stdout=subprocess.PIPE, stderr=subprocess.PIPE)


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


def request(method: str, path: str, data: bytes = b"", content_type: str | None = None) -> urllib.response.addinfourl:
    headers = {}
    if content_type is not None:
        headers["Content-Type"] = content_type
    return urllib.request.urlopen(
        urllib.request.Request(ORIGIN + path, data=data, headers=headers, method=method), timeout=10
    )


def seed_blob(repository: str, blob: bytes) -> None:
    expected = digest(blob)
    begin = request("POST", f"/v2/{repository}/blobs/uploads/")
    if begin.status != 202:
        raise RuntimeError("registry did not begin blob upload")
    location = begin.headers["Location"]
    parsed = urllib.parse.urlsplit(location)
    query = urllib.parse.parse_qsl(parsed.query, keep_blank_values=True)
    query.append(("digest", expected))
    path = parsed.path + "?" + urllib.parse.urlencode(query)
    completed = request("PUT", path, blob, "application/octet-stream")
    if completed.status != 201 or completed.headers.get("Docker-Content-Digest") != expected:
        raise RuntimeError("registry did not commit exact blob")


def seed_package(package: Path, repository: str, tag: str) -> str:
    manifest = (package / "manifest.json").read_bytes()
    config = (package / "config.json").read_bytes()
    archive = (package / "package.tar").read_bytes()
    parsed = json.loads(manifest)
    if digest(config) != parsed["config"]["digest"] or digest(archive) != parsed["layers"][0]["digest"]:
        raise RuntimeError("local build descriptors do not equal source bytes")
    seed_blob(repository, config)
    seed_blob(repository, archive)
    manifest_digest = digest(manifest)
    for reference in (manifest_digest, tag):
        response = request(
            "PUT",
            f"/v2/{repository}/manifests/{reference}",
            manifest,
            "application/vnd.oci.image.manifest.v1+json",
        )
        if response.status != 201 or response.headers.get("Docker-Content-Digest") != manifest_digest:
            raise RuntimeError("registry did not commit exact manifest")
    return manifest_digest


def normalized_result(output: Path) -> tuple[dict, dict, bytes]:
    report = json.loads((output / "verification-report.json").read_bytes())
    relationship = json.loads((output / "evidence-set-relationship.json").read_bytes())
    evidence = next((output / "evidence/sha256").iterdir()).read_bytes()
    report["invocation"]["id"] = "<invocation-id>"
    report["metadata"]["id"] = "<report-id>"
    report["metadata"]["generated_at"] = "<generated-at>"
    report["procedure"]["acquisition"] = "<truthful-source-specific-acquisition>"
    relationship["report"]["metadata"]["id"] = "<report-id>"
    return report, relationship, evidence


def main() -> None:
    workspace = Path(os.environ.get("ATTESTIFY_WORKSPACE_ROOT", Path(__file__).resolve().parents[3]))
    nape = Path(os.environ.get("NAPE_I3_BINARY", workspace / "applications/nape/target/debug/nape"))
    evaluator = workspace / "applications/nape-evaluator"
    golden = workspace / (
        "2-other-workspaces/attestify-design/attestify-product-specification/"
        "05-specs/products/verification-engine/schemas/i0-i4/conformance/golden/step-4"
    )
    i5_source = workspace / (
        "2-other-workspaces/attestify-design/attestify-product-specification/"
        "05-specs/products/verification-engine/schemas/i0-i5/conformance/source"
    )
    if not nape.is_file():
        raise RuntimeError("build applications/nape target/debug/nape before running this proof")

    with tempfile.TemporaryDirectory(prefix="attestify-i3-real-registry-") as temporary:
        root = Path(temporary)
        registry_map = root / "registry-map.yaml"
        registry_map.write_text(
            "profileVersion: attestify-oci-registry-map/1\n"
            "publishers:\n  acme.example:\n    scheme: http\n"
            f"    registry: localhost:{REGISTRY_PORT}\n"
            "    repositoryPrefix: attestify\n",
            encoding="utf-8",
        )
        root_build = root / "root-build"
        built = receipt(run([
            str(nape), "package", "build", "--source", str(i5_source / "l0-procedure"),
            "--package", ROOT_PACKAGE, "--output", str(root_build),
        ]), 0)
        manifest_digest = built["result"]["manifestDigest"]
        if seed_package(root_build, REPOSITORY, "1.0.0") != manifest_digest:
            raise RuntimeError("independent seed changed root manifest identity")

        planned = receipt(run([
            str(nape), "package", "resolve", "--package", ROOT_PACKAGE,
            "--manifest-digest", manifest_digest, "--registry-profile", str(registry_map), "--plan-only",
        ]), 0)
        if planned["result"]["dependency"] != [] or planned["result"]["root"]["disposition"] != "verified":
            raise RuntimeError("root-only plan receipt changed")

        wrapper = root / "nape-eval"
        wrapper.write_text(
            "#!/bin/sh\n" + f"export PYTHONPATH='{evaluator / 'src'}'\n" +
            f"exec python3 '{evaluator / 'main.py'}' \"$@\"\n", encoding="utf-8"
        )
        wrapper.chmod(0o700)
        build_record = root / "evaluator-build.json"
        record = canonical({
            "contract": "attestify.nape-evaluator.installed-build/v1",
            "contract_projection_digest": CONTRACT_PROJECTION,
            "dependency_set_digest": DEPENDENCY_SET,
            "evaluator_contract": "attestify.nape-evaluator.action-invocation/v2",
            "evaluator_release": "2.0.0",
            "implementation_digest": IMPLEMENTATION,
            "runner_profile": "attestify-python-test-development-v1",
        })
        build_record.write_bytes(record)
        environment = dict(os.environ)
        environment.update({
            "NAPE_EVALUATOR_V2_EXECUTABLE": str(wrapper),
            "NAPE_EVALUATOR_V2_BUILD_RECORD": str(build_record),
            "NAPE_EVALUATOR_V2_BUILD_RECORD_SHA256": digest(record),
        })
        evidence_root = root / "evidence"
        evidence_path = evidence_root / "release-readiness/database-connection/application-configuration.json"
        evidence_path.parent.mkdir(parents=True)
        shutil.copyfile(golden / "source/evidence/true/application-configuration.json", evidence_path)
        subject = golden / "source/subject.json"
        local_output = root / "local-result"
        oci_output = root / "oci-result"
        local = receipt(run([
            str(nape), "verify", "--local-package", str(root_build), "--evidence-root", str(evidence_root),
            "--subject-file", str(subject), "--output", str(local_output),
        ], environment), 0)
        oci = receipt(run([
            str(nape), "verify", "--package", ROOT_PACKAGE, "--manifest-digest", manifest_digest,
            "--registry-profile", str(registry_map), "--evidence-root", str(evidence_root),
            "--subject-file", str(subject), "--output", str(oci_output),
        ], environment), 0)
        if local["result"]["procedure"]["acquisition"]["source"] != "local-build":
            raise RuntimeError("local acquisition provenance changed")
        if oci["result"]["procedure"]["acquisition"]["source"] != "oci-pull":
            raise RuntimeError("OCI acquisition provenance is not truthful")
        if normalized_result(local_output) != normalized_result(oci_output):
            raise RuntimeError("local and OCI semantic outputs differ beyond truthful acquisition")

        already = receipt(run([
            str(nape), "package", "publish", "--local-package", str(root_build),
            "--registry-profile", str(registry_map),
        ]), 0)
        if already["result"]["disposition"] != "already-present":
            raise RuntimeError("same tag and digest was not idempotent")

        publish_source = root / "publish-source"
        shutil.copytree(i5_source / "l0-procedure", publish_source)
        publish_build = root / "publish-build"
        receipt(run([
            str(nape), "package", "build", "--source", str(publish_source),
            "--package", PUBLISHED_PACKAGE, "--output", str(publish_build),
        ]), 0)
        published = receipt(run([
            str(nape), "package", "publish", "--local-package", str(publish_build),
            "--registry-profile", str(registry_map),
        ]), 0)
        if published["result"]["disposition"] != "published":
            raise RuntimeError("absent exact package was not published")
        repeated = receipt(run([
            str(nape), "package", "publish", "--local-package", str(publish_build),
            "--registry-profile", str(registry_map),
        ]), 0)
        if repeated["result"]["disposition"] != "already-present":
            raise RuntimeError("published exact package was not subsequently idempotent")

        conflict_source = root / "conflict-source"
        shutil.copytree(publish_source, conflict_source)
        test = conflict_source / "activity/release-readiness/database-connection/database-connection.py"
        test.write_bytes(test.read_bytes() + b"\n")
        conflict_build = root / "conflict-build"
        receipt(run([
            str(nape), "package", "build", "--source", str(conflict_source),
            "--package", PUBLISHED_PACKAGE, "--output", str(conflict_build),
        ]), 0)
        conflict = receipt(run([
            str(nape), "package", "publish", "--local-package", str(conflict_build),
            "--registry-profile", str(registry_map),
        ]), 1)
        if conflict["diagnostic"]["code"] != "oci_publication_tag_conflict":
            raise RuntimeError("different content at the release tag did not conflict")

        print(canonical({
            "conflict": "passed", "idempotence": "passed", "localOciParity": "passed",
            "manifestDigest": manifest_digest, "publication": "passed", "resolvePlan": "passed",
        }).decode())


if __name__ == "__main__":
    main()
