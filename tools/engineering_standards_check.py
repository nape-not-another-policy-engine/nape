#!/usr/bin/env python3
"""Deterministic non-live structural and documentation standards checks."""

from __future__ import annotations

import json
import re
import tomllib
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]

REQUIRED_DOCS = (
    "docs/maintainers/architecture.md",
    "docs/maintainers/use-cases/build-definition-package.md",
    "docs/maintainers/use-cases/publish-definition-package.md",
    "docs/maintainers/use-cases/resolve-definition-package-plan.md",
    "docs/maintainers/use-cases/verify-procedure.md",
    "docs/maintainers/gateway-contracts.md",
    "docs/maintainers/composition.md",
    "docs/maintainers/decisions/0003-external-nape-evaluator-boundary.md",
    "docs/maintainers/decisions/0005-attestify-oci-client-library-boundary.md",
    "docs/maintainers/decisions/0006-bounded-diagnostic-outcomes.md",
    "docs/reference/evaluation-report-traceability.md",
    "docs/reference/engineering-standards-logical-paths.md",
    "docs/quality/module-quality-specifications.md",
    "docs/quality/composed-quality-assessment.md",
    "docs/quality/test-topology.md",
    "docs/maintainers/local-development.md",
    "docs/user/cli-reference.md",
    "docs/user/cli-workflow.md",
    "docs/1-plan/plans/10-engineering-standards-correction.md",
    "docs/1-plan/handoffs/10-engineering-standards-correction.md",
    "docs/1-plan/plans/12-attestify-oci-oss-release-migration.md",
    "docs/1-plan/handoffs/12-attestify-oci-oss-release-migration.md",
    "docs/1-plan/roadmap.md",
    "AGENTS.md",
    "docs/maintainers/use-cases/start-verification.md",
    "docs/maintainers/use-cases/collect-verification-evidence.md",
    "docs/maintainers/decisions/0007-nape-managed-current-verification.md",
    "docs/user/staged-verification-workflow.md",
    "docs/reference/v1-capability-to-v2-successor-matrix.md",
    "docs/reference/v1-test-successor-ledger.md",
    "docs/reference/v1-invocation-metadata-transition.md",
)

REQUIRED_MODULES = (
    "apps/nape-cli/src/composition/mod.rs",
    "apps/nape-cli/src/composition/runtime_configuration.rs",
    "apps/nape-cli/src/composition/tests.rs",
    "apps/nape-cli/src/io_adapter/clap/dispatch/mod.rs",
    "apps/nape-cli/src/io_adapter/clap/dispatch/tests.rs",
    "apps/nape-cli/src/io_adapter/clap/package_build/mod.rs",
    "apps/nape-cli/src/io_adapter/clap/package_build/tests.rs",
    "apps/nape-cli/src/io_adapter/clap/package_publish/mod.rs",
    "apps/nape-cli/src/io_adapter/clap/package_publish/tests.rs",
    "apps/nape-cli/src/io_adapter/clap/package_resolve_plan/mod.rs",
    "apps/nape-cli/src/io_adapter/clap/package_resolve_plan/tests.rs",
    "apps/nape-cli/src/io_adapter/clap/receipt/mod.rs",
    "apps/nape-cli/src/io_adapter/clap/receipt/tests.rs",
    "apps/nape-cli/src/io_adapter/clap/start/mod.rs",
    "apps/nape-cli/src/io_adapter/clap/start/tests.rs",
    "apps/nape-cli/src/io_adapter/clap/evidence/mod.rs",
    "apps/nape-cli/src/io_adapter/clap/evidence/tests.rs",
    "apps/nape-cli/src/io_adapter/clap/verify/mod.rs",
    "apps/nape-cli/src/io_adapter/clap/verify/tests.rs",
    "apps/nape-cli/tests/command_contract.rs",
    "apps/nape-cli/tests/evaluator_process.rs",
    "apps/nape-cli/tests/lifecycle_local.rs",
    "apps/nape-cli/tests/lifecycle_oci.rs",
    "apps/nape-cli/tests/lifecycle_staged.rs",
    "apps/nape-cli/src/gateway_driver/definition_package_profile_attestify_oci.rs",
)


def main() -> None:
    failures: list[str] = []

    workspace = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    members = set(workspace.get("workspace", {}).get("members", []))
    if members != {"apps/nape-cli", "domain"}:
        failures.append(f"Workspace members are {sorted(members)}")
    if workspace.get("workspace", {}).get("exclude"):
        failures.append("Workspace retains an obsolete excluded embedded dependency")

    cli_cargo = tomllib.loads(
        (ROOT / "apps/nape-cli/Cargo.toml").read_text(encoding="utf-8")
    )
    oci_dependency = cli_cargo.get("dependencies", {}).get("attestify-oci-oss")
    expected_oci_dependency = {
        "git": "ssh://git@github.com/attestify/attestify-oci-oss.git",
        "tag": "0.1.1",
    }
    if oci_dependency != expected_oci_dependency:
        failures.append("NAPE does not use the exact released attestify-oci-oss identity")
    if "attestify-oci" in cli_cargo.get("dependencies", {}):
        failures.append("NAPE aliases the OSS package through the proprietary namespace")

    lock = tomllib.loads((ROOT / "Cargo.lock").read_text(encoding="utf-8"))
    locked_oci = [
        package
        for package in lock.get("package", [])
        if package.get("name") == "attestify-oci-oss"
    ]
    expected_source = (
        "git+ssh://git@github.com/attestify/attestify-oci-oss.git?tag=0.1.1"
        "#64358b44b8a76b2abfd65f3ad1e043d615d4b6f6"
    )
    if len(locked_oci) != 1 or locked_oci[0].get("source") != expected_source:
        failures.append("Cargo.lock does not bind attestify-oci-oss 0.1.1 to its exact commit")

    retired_paths = (
        ROOT / "libraries/attestify-oci",
        ROOT / "vendor/oci-client-0.17.0",
    )
    for path in retired_paths:
        if path.exists():
            failures.append(f"superseded embedded OCI source remains: {path.relative_to(ROOT)}")

    cargo = tomllib.loads((ROOT / "domain/Cargo.toml").read_text(encoding="utf-8"))
    dependencies = set(cargo.get("dependencies", {}))
    if dependencies != {"kernel_oss"}:
        failures.append(f"Domain production dependencies are {sorted(dependencies)}")
    if set(cargo.get("dev-dependencies", {})) - {"test_framework_oss"}:
        failures.append("Domain has an unapproved development dependency")

    forbidden = re.compile(r"\b(?:serde|serde_json|serde_yaml|sha2|tokio|reqwest|oci_client)::")
    for path in sorted((ROOT / "domain/src").rglob("*.rs")):
        if forbidden.search(path.read_text(encoding="utf-8")):
            failures.append(f"Domain external capability leak: {path.relative_to(ROOT)}")

    if (ROOT / "domain/src/verification/mod.rs").exists():
        failures.append("retired Domain verification monolith remains")
    if (ROOT / "apps/nape-cli/src/verification_v2.rs").exists():
        failures.append("retired Application verification monolith remains")

    for relative in REQUIRED_DOCS:
        if not (ROOT / relative).is_file():
            failures.append(f"required document is absent: {relative}")

    for relative in REQUIRED_MODULES:
        if not (ROOT / relative).is_file():
            failures.append(f"required module is absent: {relative}")

    if (ROOT / "apps/nape-cli/tests/i2_commands.rs").exists():
        failures.append("retired monolithic command integration test remains")

    diagram_root = ROOT / "docs/assets/diagrams/plantuml"
    for number in range(1, 11):
        matches = list((diagram_root / "source").glob(f"{number:02}-*.puml"))
        if len(matches) != 1:
            failures.append(f"diagram {number:02} has {len(matches)} sources")
            continue
        for suffix in (".svg", ".png"):
            rendered = diagram_root / "rendered" / f"{matches[0].stem}{suffix}"
            if not rendered.is_file():
                failures.append(f"rendered diagram is absent: {rendered.relative_to(ROOT)}")

    ledgers = json.loads(
        (ROOT / "docs/reference/engineering-standards-c0-ledgers.json").read_text(
            encoding="utf-8"
        )
    )
    expected_counts = {
        "current_test_migration": 80,
        "historical_v1_assertion_successors": 111,
        "compatibility_proofs": 35,
    }
    for key, expected in expected_counts.items():
        if len(ledgers[key]) != expected:
            failures.append(f"{key} has {len(ledgers[key])} rows, expected {expected}")

    rust_roots = (
        ROOT / "domain/src",
        ROOT / "apps/nape-cli/src",
        ROOT / "apps/nape-cli/tests",
    )
    test_pattern = re.compile(
        r"#\[(?:tokio::)?test\]\s+(?:async\s+)?fn\s+([A-Za-z0-9_]+)\("
    )
    for rust_root in rust_roots:
        for path in sorted(rust_root.rglob("*.rs")):
            source = path.read_text(encoding="utf-8")
            if re.search(r"(?<![A-Za-z0-9_])attestify_oci::", source):
                failures.append(
                    f"proprietary OCI namespace remains in OSS consumer: {path.relative_to(ROOT)}"
                )
            if ".unwrap()" in source or ".unwrap_err()" in source:
                failures.append(f"test unwrap remains: {path.relative_to(ROOT)}")
            for name in test_pattern.findall(source):
                if not name.endswith(("_success", "_error", "_success_async", "_error_async")):
                    failures.append(f"nonconformant test name {name} in {path.relative_to(ROOT)}")

    if failures:
        raise SystemExit("\n".join(failures))
    print(
        "verified exact attestify-oci-oss release binding, retired embedded source, "
        "Domain dependency/ownership rules, 31 documents, exact bounded module "
        "paths, ten diagram source/render pairs, frozen ledger counts, and Rust "
        "test form"
    )


if __name__ == "__main__":
    main()
