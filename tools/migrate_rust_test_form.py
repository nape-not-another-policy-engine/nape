#!/usr/bin/env python3
"""One-time deterministic migration to the repository Rust test form."""

from __future__ import annotations

import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SEARCH_ROOTS = (
    ROOT / "domain/src",
    ROOT / "apps/nape-cli/src",
    ROOT / "apps/nape-cli/tests",
)
ERROR_WORDS = (
    "error",
    "reject",
    "invalid",
    "fail",
    "cannot",
    "missing",
    "mismatch",
    "nonzero",
    "timeout",
    "exceed",
    "absent",
    "mutation",
    "tamper",
    "conflict",
    "duplicate",
    "unsorted",
    "incomplete",
    "noncanonical",
)
PATTERN = re.compile(
    r"(?m)^(?P<indent>\s*)#\[(?P<attribute>(?:tokio::)?test)\]\n"
    r"(?:(?P<doc>\s*/// Requirement validation:[^\n]*\n))?"
    r"(?P<fnindent>\s*)(?P<async>async\s+)?fn\s+(?P<name>[a-zA-Z0-9_]+)\("
)


def replacement(match: re.Match[str]) -> str:
    name = match.group("name")
    asynchronous = match.group("async") is not None
    valid_suffixes = ("_success", "_error", "_success_async", "_error_async")
    if not name.endswith(valid_suffixes):
        outcome = "error" if any(word in name for word in ERROR_WORDS) else "success"
        name = f"{name}_{outcome}{'_async' if asynchronous else ''}"
    doc = match.group("doc") or (
        f"{match.group('indent')}/// Requirement validation: exercises one bounded logical path.\n"
    )
    asynchronous_text = match.group("async") or ""
    return (
        f"{doc}{match.group('indent')}#[{match.group('attribute')}]\n"
        f"{match.group('fnindent')}{asynchronous_text}fn {name}("
    )


def main() -> None:
    for search_root in SEARCH_ROOTS:
        for path in sorted(search_root.rglob("*.rs")):
            source = path.read_text(encoding="utf-8")
            migrated = PATTERN.sub(replacement, source)
            if migrated != source:
                path.write_text(migrated, encoding="utf-8")


if __name__ == "__main__":
    main()
