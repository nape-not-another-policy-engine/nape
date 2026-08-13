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
  Immutable V1 contract and capability evidence. Its Assurance documents,
  command grammar, evaluator JSON, and state forms are not current interfaces.

## Where We Left Off

The bounded I0-through-I6 functional delivery is complete. NAPE CLI 2.0 can
build, publish, pull, verify, execute, and locally persist the complete current
Verification V2 lifecycle through embedded and referenced Procedure, Activity,
and Action compositions.

The Engineering and Rust Test Standards correction is complete. It
preserves the completed V2 lifecycle and every implemented V1 user capability
as current Verification V2 behavior, including staged Start, exactly-one-file-
per-call Evidence collection, repeated-command evidence correction, invocation
metadata, evaluation, and local reporting. Current execution replaces
`AssuranceProcedure` and `AssuranceReport` with `VerificationProcedure` and
`VerificationReport` V2. The exact domain-neutral `attestify-oci-oss` `0.1.1`
release is now the sole NAPE OCI library dependency. The superseded embedded
library and patched OCI-client source are retired after complete pre-removal
and post-removal compatibility proofs.

## Active Workstreams

### Current Attestify OCI OSS transport correction

NAPE now consumes exact signed release `attestify-oci-oss` `0.1.1` at commit
`64358b44b8a76b2abfd65f3ad1e043d615d4b6f6`. The release removes an
OCI-unspecified blob-download response `Content-Type` restriction while
preserving manifest media-type admission and all digest, size, encoding,
redirect, destination, timeout, and byte-bound controls. NAPE retains every
Verification-specific profile, command, translation, and test. The complete
candidate and exact-release NAPE suites are required to pass before this
bounded dependency migration closes.

### Completed Plan 12: Attestify OCI OSS Release Migration

Status:

- corrected `attestify-oci-oss` commit
  `013bb352366fcad80db682984705911ca2108258` is released as signed tag
  `0.1.0`;
- the complete exact-release pre-removal NAPE suite passed, including its
  bounded localhost transport tests;
- NAPE now depends only on that exact OSS release and retains its
  Verification-specific profile and adapters;
- the superseded embedded OCI library and patched OCI-client source are
  removed; and
- the complete post-removal in-place and clean composite suites pass.

Purpose:

- replace NAPE's embedded OCI mechanics with the smallest reusable,
  domain-neutral OSS release;
- preserve all NAPE-owned Verification semantics and behavior; and
- prove exact-release compatibility before and after retiring the embedded
  implementation.

Plan:

- `docs/1-plan/plans/12-attestify-oci-oss-release-migration.md`

Handoff:

- `docs/1-plan/handoffs/12-attestify-oci-oss-release-migration.md`

Central authority:

- `../../../../1-attestify-product-workspace-docs/0-plans/attestify-workflow-harness/harness-specification/workflow-definition-authoring/208-wpc-m1-oss-release-and-nape-migration-material-stop-checkpoint.md`

Stop boundary:

- NAPE migration is complete;
- proprietary OCI remains deferred until a concrete non-NAPE requirement;
- Workflow package and catalog OCI semantics remain consumer-owned WPC-M1
  work.

### Completed Plan 11: WPC-M1 Slice 1 Current-Source Qualification

Status:

- authorized by Workflow Harness record 203 on 2026-08-06
- C1 through C3 are complete through Workflow Harness record 204
- the Attestify OCI active-planning reconciliation and contradiction gate pass
- record 204 and WPC-M1 Slice 1 were accepted and closed on 2026-08-06
- the original Slice 1 qualification ended without source extraction or
  migration; those later actions are governed by Plan 12 and central records
  205 through 208

Purpose:

- preserve the exact current uncommitted `attestify-oci` and patched
  `oci-client` source by deterministic manifest identity;
- black-box the five patched raw-response methods and their current closed
  NAPE-facing adapter operations against a bounded loopback HTTP double;
- freeze the OSS, proprietary, and NAPE ownership boundary and later migration
  order without performing that migration; and
- record the accepted WPC-M1 Slice 1 closure without beginning migration.

Plan:

- `docs/1-plan/plans/11-wpc-m1-slice-1-current-source-qualification.md`

Handoff:

- `docs/1-plan/handoffs/11-wpc-m1-slice-1-current-source-qualification.md`

Closure authority:

- `../../../../1-attestify-product-workspace-docs/0-plans/attestify-workflow-harness/harness-specification/workflow-definition-authoring/204-wpc-m1-slice-1-c1-c3-completion-checkpoint-and-coordination-material-stop.md`

Stop boundary:

- stop after the Slice 1 closure recommendation;
- do not begin record-203 M1 through M6 or original WPC-M1 Slice 2.

### Completed Plan 10: NAPE Engineering Standards Correction

Status:

- V1 capability-parity and simple-interface reconciliation complete
- exactly one top-level lifecycle: `nape start`, repeated one-file
  `nape evidence`, and argument-free `nape verify`
- continuous C0-through-C6 implementation authorized on 2026-08-03
- C0 characterization, ownership, identity, and parity freeze complete
- C1 Kernel/value/diagnostic/parser foundations and bounded Product metadata
  amendment complete
- C2 six Domain Use Cases and deterministic subordinate services complete
- C3 Application Gateway drivers, reusable `attestify-oci` ownership, and
  restartable verified package custody complete
- all eight C3 gates pass; 134 workspace Rust tests pass
- C4 thin CLI adapters, composition, and staged command cutover complete
- C5 module, test, documentation, and non-live quality conformance complete
- C6 corrected and proved Receipt V2 publish-tag versus resolve-digest
  projection and private Evaluator-workspace custody
- C6 added and proved mutually exclusive current Evaluator V2/V3 Verification
  Report selection and the current Evidence Set relationship selector
- all eight correction gates and two final clean-registry lifecycles pass

Purpose:

- introduce the required Use Case, Gateway, bounded request/response,
  I/O-adapter, composition, client-library, module-quality, documentation, and
  Rust Test Standard structure
- preserve current V2 package, Evaluator, existing Receipt, Report,
  relationship, registry, continuation, and atomic local-persistence behavior
- add staged Verification Start and one-piece-at-a-time Evidence operations
  with V1's silent success behavior; retain the governed Verify Receipt V2
  projection
- preserve V1 invocation metadata through one bounded Verification Report V2
  amendment and preserve evidence correction by repeating the Evidence
  command with atomic replacement

Plan:

- `docs/1-plan/plans/10-engineering-standards-correction.md`

Handoff:

- `docs/1-plan/handoffs/10-engineering-standards-correction.md`

Central authority:

- `../../../../1-attestify-product-workspace-docs/0-plans/attestify-oci/nape-cli-v2-engineering-standards-correction-plan.md`
- `../../../../1-attestify-product-workspace-docs/0-plans/attestify-oci/nape-c3-attestify-oci-and-nape-ownership-correction-plan.md`
- `../../../../1-attestify-product-workspace-docs/0-plans/attestify-oci/nape-v1-capability-to-verification-v2-reconciliation-audit.md`
- `../../../../1-attestify-product-workspace-docs/0-plans/attestify-oci/nape-v1-capability-parity-and-verification-v2-interface-review.md`

Stop boundary:

- complete C0 through C6 and all eight standards-correction gates
- stop before the OCI Profile `0.1.0` snapshot and all excluded post-I6 work

### Completed Plan 09: I6-F01 Through I6-F06 Functional Delivery

Status:

- approved, complete, and frozen
- F01 through F06 promoted under the development qualification
- both approved current-I6 Receipt capacity corrections are frozen
- real Evaluator V3 and two-clean-registry qualification passes
- all Product, NAPE, Evaluator, projection, and predecessor regression gates
  pass

Purpose:

- complete the NAPE Verification Engine V2 functional delivery through
  validated structured evidence, package-owned evaluations, authorized
  criteria assignment, declared helper modules, the feature-rich Action and
  Procedure compositions, and fixed mixed-state continuation
- preserve NAPE V1, I0-I5 behavior, Evaluator V2, and the I5 lifecycle while
  adding only the approved Evaluator V3 and development-v2 functional path

Plan:

- `docs/1-plan/plans/09-i6-f01-f06-functional-delivery.md`

Handoff:

- `docs/1-plan/handoffs/09-i6-f01-f06-functional-delivery.md`

Central authority and program record:

- `../../../../1-attestify-product-workspace-docs/0-plans/attestify-oci/attestify-verification-engine-oci-i6-f01-through-f07-planning-pass-3.md`
- `../../../../1-attestify-product-workspace-docs/0-plans/attestify-oci/attestify-verification-engine-i6-f01-f06-program-integration-record.md`

Stop boundary:

- stop after F06 and the holistic program completion review
- do not begin F07 Fact Finding, Risk, UI/outcome persistence, catalog,
  enterprise hardening, authenticated registry, client, or Harness work

### Completed Plan 08: I5 Full Developer Lifecycle

Status:

- complete
- I5-S1 complete
- I5-S2 complete
- I5-S3 complete
- I5-S4 complete
- I5-S5 complete

Purpose:

- deliver the complete unauthenticated developer lifecycle through NAPE for
  L0 embedded, L1 referenced-Action, and L2 referenced-Activity topologies
- build every accepted package through NAPE, publish the exact build results,
  pull and verify the complete closure, execute each Action occurrence through
  the unchanged Evaluator V2 boundary, and atomically expose the complete
  local result directory

Plan:

- `docs/1-plan/plans/08-i5-full-developer-lifecycle.md`

Handoff:

- `docs/1-plan/handoffs/08-i5-full-developer-lifecycle.md`

Central authority and living record:

- `../../../../1-attestify-product-workspace-docs/0-plans/attestify-oci/attestify-verification-engine-oci-i5-bounded-implementation-plan.md`
- `../../../../1-attestify-product-workspace-docs/0-plans/attestify-oci/attestify-verification-engine-oci-i5-implementation-record.md`
- `../../../../1-attestify-product-workspace-docs/0-plans/attestify-oci/attestify-verification-engine-oci-i5-completion-review.md`

Stop boundary:

- stop after the holistic I5 completion review
- do not begin authentication, trust, hardened Runner work, OCI Report
  publication, functional I6, Risk, client, Harness, a NAPE commit, or a
  repository-visibility decision

### Completed I0-I4 OCI Delivery

The bounded I0-I4 delivery is complete and stopped before post-I4 behavior.
Its repository-local Plans 02 through 07 and matching handoffs are preserved
byte-for-byte in [central I0-I4 history](../../../../1-attestify-product-workspace-docs/0-plans/attestify-oci/history/i0-i4/README.md).

Current NAPE behavior remains governed by the production source, current
contracts, current conformance, and the central completion records. The
historical plans are provenance, not current test or execution entry points.

### Plan 01: Evaluator Output Integration Into NAPE CLI

Status:

- paused legacy/current-public integration history
- versioned `/v2` work moved to Plan 04 without rewriting Plan 01 history

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

Return to Workflow Harness WPC-M1 for consumer-owned Workflow authoring,
qualification, publication, locked catalog release, OCI acquisition,
activation, discovery, and durable consumption. Fact Finding, Risk, outcome
UI or non-local persistence, authentication, trust, enterprise Runner
hardening, OCI Report publication, and unrelated client work remain outside
the completed NAPE migration.

## Operating Rule

If a new `nape` workstream appears:

1. add it here first
2. create its plan doc
3. create its handoff doc
4. update `AGENTS.md` only if the start order or operating rules change
