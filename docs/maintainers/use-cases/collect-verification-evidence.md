# Collect Verification Evidence Use Case

## What and why

`CollectVerificationEvidenceUC` adds or replaces exactly one evidence file for one effective Action occurrence in the NAPE-managed current run. One-file-at-a-time collection preserves the V1 user capability without exposing mutable execution state.

## Request and outcome

The request contains one canonical `activity.action` selector, one opaque source-file handle, and an optional definition-filename assertion. The source basename may differ from the definition-owned filename. Completion returns the action association, digest, byte count, and whether the physical payload was newly stored.

## Flow and invariants

NAPE reopens the current collecting run, selects the one definition-owned evidence requirement, performs a stable regular-file read under the effective ceiling, and atomically commits the association. Equal bytes deduplicate by digest. Repeating the same association is idempotent; different bytes replace only that occurrence. No Test executes. A completed run is immutable.

CLI mapping: `nape evidence`. Logical paths: add, idempotent repeat, atomic replacement, shared payload, wrong filename assertion, absent occurrence, oversize/unstable input, and completed-run rejection.

![Current-run state](../../assets/diagrams/plantuml/rendered/10-current-run-state.svg)

Source: [10-current-run-state.puml](../../assets/diagrams/plantuml/source/10-current-run-state.puml)
