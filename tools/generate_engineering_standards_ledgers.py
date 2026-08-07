#!/usr/bin/env python3
"""Generate deterministic Markdown views of the frozen C0 successor ledgers."""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "docs/reference/engineering-standards-c0-ledgers.json"


def escape(value: object) -> str:
    return str(value).replace("|", "\\|").replace("\n", " ")


def main() -> None:
    data = json.loads(SOURCE.read_text(encoding="utf-8"))
    historical = data["historical_v1_assertion_successors"]

    matrix = [
        "# V1 Capability To V2 Successor Matrix",
        "",
        "The Assurance-era command and representation are retired, but their user capabilities have explicit Verification V2 successors. The frozen assertion-level evidence is listed below; several historical tests may point to the same current proof because they asserted the same business capability.",
        "",
        "| V1 area | Verification V2 successor |",
        "| --- | --- |",
        "| Git Procedure retrieval | exact local-build or OCI package acquisition; Git transport rejected |",
        "| AssuranceProcedure parsing | controlled VerificationProcedure/Activity/Action admission |",
        "| start collection | `nape start` and one atomically selected current run |",
        "| collect evidence | repeated one-file `nape evidence`, stable read, digest storage, atomic association |",
        "| evaluate evidence | argument-free `nape verify` over frozen current-run inputs |",
        "| AssuranceReport | VerificationReport plus Evidence Set relationship and raw evidence |",
        "| mutable run files | complete versioned current-run state and atomic commits |",
        "| caller metadata | bounded top-level Report metadata with reserved Engine keys |",
        "| output discovery | NAPE-managed result location returned by Verify Receipt |",
        "",
        "No `nape collect`, Assurance artifact, Git/HTTPS repository, one-shot Verify, caller workspace, caller output, or caller-selected run handle is a current compatibility path.",
    ]
    (ROOT / "docs/reference/v1-capability-to-v2-successor-matrix.md").write_text(
        "\n".join(matrix) + "\n", encoding="utf-8"
    )

    ledger = [
        "# V1 Test Successor Ledger",
        "",
        "This is the deterministic Markdown view of the 111 active V1 tests pinned at commit `4c13c74`. The JSON C0 ledger remains the machine-readable source.",
        "",
        "| ID | Historical source and test | Assertions | Successor proof | Disposition |",
        "| --- | --- | ---: | --- | --- |",
    ]
    for row in historical:
        source = f"`{row['file']}:{row['line']}` `{row['function']}`"
        proofs = ", ".join(f"`{proof}`" for proof in row["successor_proofs"])
        disposition = f"{row['successor_owner']}. {row['obsolete_mechanism_rationale']}"
        ledger.append(
            f"| {escape(row['id'])} | {escape(source)} | {row['assertion_count']} | {escape(proofs)} | {escape(disposition)} |"
        )
    ledger.extend(
        [
            "",
            f"Total rows: {len(historical)}. No historical test is executed merely because it exists; current acceptance uses its named successor proof or an explicit obsolete-mechanism disposition.",
        ]
    )
    (ROOT / "docs/reference/v1-test-successor-ledger.md").write_text(
        "\n".join(ledger) + "\n", encoding="utf-8"
    )


if __name__ == "__main__":
    main()
