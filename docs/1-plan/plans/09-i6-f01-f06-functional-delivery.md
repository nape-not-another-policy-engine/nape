# Plan 09: I6-F01 Through I6-F06 Functional Delivery

Status: approved, complete, and frozen under the development qualification

## Outcome

Complete the bounded functional Verification Engine V2 delivery after I5:

1. F01 validated structured evidence.
2. F02 package-owned evaluations.
3. F03 authorized criteria assignment.
4. F04 declared helper modules.
5. F05A feature-rich Profile-1 Action.
6. F05B feature-rich Profile-1 Procedure.
7. F06 mixed execution states and fixed continuation.

## Authority

The central approved terminal plan and program record are the authority. This
repository plan transcribes them and does not reopen their semantics.

## CP0 seam

- Preserve `domain/src/verification/mod.rs` and
  `apps/nape-cli/src/verification_v2.rs` as exclusive integrator-owned
  facades while extracting feature-owned modules beneath them.
- Preserve every I0-I5 admission function as a compatibility wrapper.
- Add a promoted capability set so existing rejection boundaries are removed
  only for the exact implemented feature set.
- Add a version-neutral evaluator adapter above exact V2 and additive V3
  providers.
- Preserve all-inputs-frozen-before-first-execution and the existing atomic
  result store.

## Promotion order

```text
CP0 -> F01/F02/F03/F04 -> F05A -> F05B -> F06 -> stop
```

Feature module work may overlap, but promotion requires the real predecessor
implementation. Final proof may not substitute mocks.

## Important translations

- Local/OCI parity uses identical NAPE-built package bytes; it is not local
  filesystem dependency resolution.
- Current authored source omits `attestify-lock.json`; NAPE generates Locks.
- Historical A1-A8 Locks remain historical fixture content only.

## Completion

Each feature uses its approved five slices and eight gates. The program closes
only when F01-F06 and all predecessor regression, local/OCI parity, Report,
Evidence Set, raw-evidence, and two-clean-run obligations pass.

## Current implementation result

- The additive functional domain, V3 consumer, Procedure/Activity/Action
  resolver, structured-evidence admission, assignment authority, module
  preflight, occurrence continuation, I6 Report, Evidence Set, and atomic raw-
  evidence result are implemented.
- Historical I0-I5 selection still uses Evaluator V2 and development-v1;
  functional selection uses exact V3 and development-v2.
- Focused domain, V2/V3 boundary, mixed-state, terminal non-promotion, and full
  workspace predecessor suites pass.
- The qualification tool regenerates current A6/A7 sources without authored
  Locks or historical fact-null values, builds twice, and targets two clean
  registries plus the real Evaluator V3 provider.

Both approved current-I6 Receipt capacity corrections are closed. Receipt V2
admits zero or more unique Edge rows and uses the exact I6 Report summary
envelope, while the Lock and 64-KiB Receipt limits remain authoritative. The
exact three-Edge Resolve Receipt and two-Activity/seven-Action Verify Receipt
validate.

The approved conceptual-to-current source translation prepares the five A7
opaque Tests with explicit strict UTF-8 decoding and deterministic missing-key
handling while preserving every historical example byte. Final Action,
Activity, and Procedure manifests are pinned.

The final qualification passes deterministic double builds, exact publication,
complete three-Edge resolution, seven real Evaluator V3 calls, complete local
Report/Evidence Set/raw-evidence persistence, and equal normalized outcomes
through two clean registries. The NAPE workspace passes 187 tests, the
Evaluator passes 194 tests with ten expected exact-host skips, the Product
validator and both projections pass, and frozen I0-I5 predecessors replay.

## Exclusions

No F07, Fact Finder, Evaluator V4, Risk, catalog, outcome persistence/UI,
independent library packages, credentials/trust, hardened Runner, OCI result
publication, client, Harness, commit, or visibility decision.
