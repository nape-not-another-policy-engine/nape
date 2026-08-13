# Plan 08: I5 Full Developer Lifecycle

Status: complete; all five slices, 38 proof rows, and eight gates passed

## Outcome

NAPE delivers one complete developer-operated lifecycle for three finite
topologies:

```text
source -> build -> publish -> pull/verify closure -> execute -> local result
```

- L0: one embedded-Action Procedure;
- L1: one Procedure referencing one independently built Action; and
- L2: one Procedure referencing one independently built Activity containing
  one embedded and one referenced Action.

## Authority

The complete decisions, proof rows, gates, exclusions, and material-change
stop rule are in the central
[I5 implementation plan](../../../../../1-attestify-product-workspace-docs/0-plans/attestify-oci/attestify-verification-engine-oci-i5-bounded-implementation-plan.md).
This repository-local plan transcribes that authority; it does not reopen it.

## Ordered slices

1. `I5-S1`: additive contracts, repeatable dependency input, exact source
   projections, and planning seams.
2. `I5-S2`: deterministic Action, Activity, and Procedure construction with
   builder-generated canonical Locks.
3. `I5-S3`: exact publication, zero/one/two-Edge planning, and complete OCI
   closure acquisition.
4. `I5-S4`: complete evidence admission, ordered execution, origin-complete
   reporting, and atomic local persistence.
5. `I5-S5`: L0-L2 lifecycle qualification, clean-registry repetition,
   regressions, exact identities, and completion review.

## Repository ownership

- NAPE owns definition semantics, effective occurrences, evidence admission,
  Evaluator orchestration, continuation, Reports, relationships, receipts,
  and atomic output promotion.
- `attestify_oci` owns package construction mechanics, Lock and envelope
  verification, PURL-to-registry mapping, exact publication, and exact
  acquisition.
- NAPE Evaluator remains OCI-unaware and executes one prepared Action request
  at a time through the frozen I1 ABI.

## Completion

All 38 proof rows and eight gates in the central plan must pass. After every
slice, record whether there is a contract contradiction, material downstream
change, outside authority requirement, or failed applicable gate. Continue
only when all four answers permit it.

I5 completed on 2026-07-31. The authoritative results are in the central
[I5 completion review](../../../../../1-attestify-product-workspace-docs/0-plans/attestify-oci/attestify-verification-engine-oci-i5-completion-review.md).

## Exclusions

No authentication, credentials, trust, records, cache, external PURL
registration, hardened Runner work, OCI Report publication, functional I6,
Risk, client, Harness, NAPE commit, or visibility decision.
