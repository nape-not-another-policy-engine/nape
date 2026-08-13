#!/usr/bin/env python3
"""Validate current NAPE CLI 2.0 documentation navigation."""

from __future__ import annotations

import re
from pathlib import Path
from urllib.parse import unquote


ROOT = Path(__file__).resolve().parents[1]
LINK = re.compile(r"\[[^\]]*\]\(([^)]+)\)")


def current_markdown() -> list[Path]:
    result = [ROOT / "README.md"]
    result.extend(
        path
        for path in (ROOT / "docs").rglob("*.md")
        if "history" not in path.parts and "1-plan" not in path.parts
    )
    return sorted(result)


def main() -> None:
    failures: list[str] = []
    documents = current_markdown()
    for document in documents:
        text = document.read_text(encoding="utf-8")
        for obsolete in (
            "nape collect start",
            "nape collect evidence",
            "nape collect report",
        ):
            if obsolete in text:
                failures.append(f"current V1 command in {document.relative_to(ROOT)}")
        for match in LINK.finditer(text):
            target = match.group(1).split("#", 1)[0].strip()
            if not target or "://" in target or target.startswith("mailto:"):
                continue
            path = (document.parent / unquote(target)).resolve()
            try:
                path.relative_to(ROOT)
            except ValueError:
                failures.append(
                    f"link escapes repository in {document.relative_to(ROOT)}: {target}"
                )
                continue
            if not path.exists():
                failures.append(
                    f"missing link in {document.relative_to(ROOT)}: {target}"
                )

    historical = ROOT / "docs/history/v1"
    required_history = (
        historical / "repository-readme.md",
        historical / "product/v1-verification-baseline.md",
        historical / "reference/v1-contract-matrix.md",
        historical / "user/cli-reference.md",
        historical / "maintainers/architecture.md",
        historical / "scripts/docs-rover-smoke.sh",
    )
    for path in required_history:
        if not path.is_file():
            failures.append(f"missing V1 history: {path.relative_to(ROOT)}")

    examples = ROOT / "docs/examples/verification-v2-oci"
    if any(path.name == "attestify-lock.json" for path in examples.rglob("*")):
        failures.append("checked authored source contains attestify-lock.json")

    if failures:
        raise SystemExit("\n".join(failures))
    print(
        f"verified {len(documents)} current Markdown documents, local links, "
        "V2-only commands, V1 history anchors, and lock-free checked source"
    )


if __name__ == "__main__":
    main()
