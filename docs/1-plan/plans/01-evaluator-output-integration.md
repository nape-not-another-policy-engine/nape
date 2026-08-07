# Plan 01: Evaluator Output Integration Into NAPE CLI

Status: **paused legacy/current-public integration history**

The versioned
`attestify.nape-evaluator.action-invocation/v2` consumer is owned by Plan 04.
Do not extend this plan to implement that boundary. Preserve this document as
the historical/current-public seam it originally described.

## Goal

Update `nape` so the Rust CLI can consume the updated `nape-evaluator` output contract and map it into the current report-building flow deliberately instead of relying on the older flat `outcome`/`reason` JSON object.

## Baseline

Read first:

1. `docs/1-plan/roadmap.md`
2. `docs/maintainers/architecture.md`
3. `docs/maintainers/decisions/0003-external-nape-eval-boundary.md`
4. `docs/reference/evaluation-report-traceability.md`
5. `docs/reference/v1-contract-matrix.md`
6. sibling `../nape-evaluator/docs/1-plan/roadmap.md`
7. sibling `../nape-evaluator/docs/product/current-evaluator-reference.md`
8. sibling `../nape-evaluator/docs/product/v2-structured-verification-result-proposal.md`

## Current State

Current committed `nape` behavior:

- the adapter shells out to `nape-eval`
- it verifies install with `nape-eval --check-install`
- it invokes one evaluator call per evidence/test pair
- it deserializes stdout into a local struct containing only:
  - `outcome`
  - `reason`
- it converts those two fields into domain `TestResult`

Current committed domain model:

- `EvaluationResults` stores one `TestResult` per evidence/test pair
- `TestResult` currently contains:
  - `outcome`
  - `reason`

## Scope

In scope:

- evaluator adapter deserialization changes
- explicit mapping from evaluator output into current domain result objects
- tests for the adapter and report flow
- documentation and ADR updates at the evaluator seam

Out of scope:

- replacing the external evaluator boundary
- unrelated procedure/repository retrieval changes
- full report-schema redesign unless the integration work proves that it is necessary

## Work Plan

1. Confirm the evaluator contract being integrated.
   Do not implement against an ambiguous proposal snapshot. Lock the exact evaluator output shape first.

2. Inventory current `nape` assumptions.
   Capture every place that assumes evaluator stdout is only `{outcome, reason}`.

3. Decide the integration translation model.
   Decide what `nape` consumes directly from the evaluator envelope, what it ignores for now, and what it preserves for future report evolution.

4. Update the adapter.
   Change `evaluate_evidence_gateway.rs` to parse the newer evaluator output shape safely.

5. Update domain/report mapping if needed.
   If the new evaluator shape forces richer action-level semantics, make that explicit instead of hiding it inside ad hoc parsing.

6. Update tests.
   Adapter tests, use-case tests, and any report-related tests should reflect the chosen evaluator contract.

7. Update docs.
   The ADR, architecture doc, contract matrix, and traceability doc must match the integrated behavior.

## Primary Design Questions

- What is the minimum subset of the new evaluator output that `nape` must consume immediately?
- How should `nape` treat blocked execution versus executed test results?
- Should evaluator messages remain CLI-internal diagnostics at first, or should they influence report content?
- If `nape-evaluator` moves toward structured result payloads, does `nape` flatten them back to the current report shape or start carrying richer action data?
- Does the current `TestResult` domain type remain sufficient, or does it need an explicit V2 expansion step?

## Done Criteria

This plan is ready to close when:

- `nape` can parse the chosen updated evaluator output contract
- the adapter and tests are aligned with the new evaluator output shape
- the current report impact is documented clearly
- permanent `nape` docs explain the new evaluator seam accurately
