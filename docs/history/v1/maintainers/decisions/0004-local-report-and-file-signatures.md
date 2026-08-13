# ADR 0004: Local Report Output And File Signatures

Status: current V1 behavior

## Context

`nape collect report` writes a local YAML report to:

```text
<run-home>/assurance_report.yaml
```

For every evaluated action, NAPE signs the test file and evidence file with SHA256 and records the signatures in the report.

## Decision

V1 produces local reports and records signatures for the exact test and evidence bytes used during evaluation.

## Consequences

Positive:

- Users can inspect reports without a server or upload target.
- Report readers can verify which evidence and test bytes were evaluated.
- Local examples and smoke runs are straightforward.

Negative:

- Current CLI help still mentions upload behavior that is not implemented.
- Reports do not include a first-class remote artifact location.
- Reports use SHA256 only.

## V2 Considerations

V2 should decide whether local reports remain primary, whether upload becomes explicit behavior, and whether signatures should include algorithm metadata, canonical path metadata, or source repository commit metadata.

