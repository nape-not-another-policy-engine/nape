# Handoff 01: Evaluator Output Integration Into NAPE CLI

Status: **paused legacy/current-public integration history**

Resume the versioned
`attestify.nape-evaluator.action-invocation/v2` consumer from Plan 04/Handoff
04. This handoff remains the historical record for the older flat-output
integration seam.

## Purpose

Resume `nape` work at the exact point where it paused while `nape-evaluator` input/output work advanced in the sibling repository.

## Read Order

1. `docs/1-plan/roadmap.md`
2. `docs/1-plan/plans/01-evaluator-output-integration.md`
3. `docs/maintainers/architecture.md`
4. `docs/reference/evaluation-report-traceability.md`
5. `docs/maintainers/decisions/0003-external-nape-eval-boundary.md`
6. sibling `../nape-evaluator/docs/1-plan/roadmap.md`
7. sibling `../nape-evaluator/docs/product/current-evaluator-reference.md`
8. sibling `../nape-evaluator/docs/product/v2-structured-verification-result-proposal.md`

## Where We Paused

`nape` paused while `nape-evaluator` planning and proposal work moved forward.

The concrete paused seam is:

- `apps/nape-cli/src/gateway_adapter/nape_evaluator/evaluate_evidence_gateway.rs`

That adapter still assumes evaluator stdout is:

```json
{
  "outcome": "pass",
  "reason": "Reason text"
}
```

The current gateway test still reflects older evaluator assumptions and does not yet exercise the newer evaluator envelope direction.

## Resume Focus

When resuming this plan, focus on:

1. locking the exact evaluator output contract to integrate
2. deciding the translation layer from evaluator output into current `nape` domain/report semantics
3. updating the adapter and tests deliberately, not incrementally by guesswork
4. documenting any temporary flattening or ignored evaluator fields explicitly

## Guardrails

- do not change `nape` against an unstable evaluator shape
- do not hide blocked execution semantics behind the old flat `outcome`/`reason` assumption
- do not update only code or only docs; the evaluator seam documentation must stay synchronized
- do not start broader report-schema redesign in this repo unless the integration work proves it is required

## Next Useful Outputs

- a locked integration contract note for the evaluator seam
- updated adapter deserialization and translation tests
- ADR and traceability updates that explain how `nape` is consuming the new evaluator output
