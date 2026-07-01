# NAPE Roadmap

This roadmap is the planning index for `nape`-specific work.

## Start Order

When resuming `nape` work:

1. read `docs/1-plan/roadmap.md`
2. read the active plan document
3. read the matching handoff document
4. re-ground on `docs/maintainers/architecture.md`
5. re-ground on `docs/reference/evaluation-report-traceability.md`

If the work touches the evaluator seam, also re-ground on the sibling `nape-evaluator` roadmap and current evaluator reference.

## Baseline Already Established

The following baseline work is already in place and should be treated as prerequisite context:

- `docs/maintainers/architecture.md`
  Current CLI, domain, and adapter architecture.
- `docs/maintainers/decisions/0003-external-nape-eval-boundary.md`
  Current external evaluator boundary.
- `docs/reference/evaluation-report-traceability.md`
  Current path from procedure inputs through evaluator calls into report fields.
- `docs/reference/v1-contract-matrix.md`
  Current V1 contracts, including the evaluator JSON contract expected by `nape`.

## Where We Left Off

Work paused in `nape` at the external evaluator integration seam while `nape-evaluator` planning and output/input expansion moved into the sibling repository.

The concrete paused seam is:

- `apps/nape-cli/src/gateway_adapter/nape_evaluator/evaluate_evidence_gateway.rs`

Current `nape` behavior at that seam:

- runs `nape-eval --check-install`
- runs `nape-eval --evidence <evidence-file> --test <test-file>`
- expects evaluator stdout JSON shaped as:

```json
{
  "outcome": "pass",
  "reason": "Reason text"
}
```

That is now the primary follow-up area in `nape` once the `nape-evaluator` output direction is ready to integrate.

## Active Workstreams

### Plan 01: Evaluator Output Integration Into NAPE CLI

Status:

- active

Purpose:

- update the Rust CLI evaluator adapter, domain mapping, tests, and docs so `nape` can consume the newer `nape-evaluator` output contract cleanly

Plan:

- `docs/1-plan/plans/01-evaluator-output-integration.md`

Handoff:

- `docs/1-plan/handoffs/01-evaluator-output-integration.md`

Primary dependencies:

- `docs/maintainers/decisions/0003-external-nape-eval-boundary.md`
- `docs/reference/evaluation-report-traceability.md`
- sibling `nape-evaluator`:
  - `docs/1-plan/roadmap.md`
  - `docs/product/current-evaluator-reference.md`
  - `docs/product/v2-structured-verification-result-proposal.md`

## Next Likely Follow-Up

After Plan 01 stabilizes, the next likely `nape` work is:

- report and contract alignment if `nape-evaluator` introduces richer execution or result semantics that should surface beyond the current `action[].outcome` and `action[].reason` fields

This is not an active plan yet. Create it only after the evaluator integration direction is explicit enough.

## Operating Rule

If a new `nape` workstream appears:

1. add it here first
2. create its plan doc
3. create its handoff doc
4. update `AGENTS.md` only if the start order or operating rules change
