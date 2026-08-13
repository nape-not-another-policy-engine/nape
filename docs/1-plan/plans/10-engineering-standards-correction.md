# Plan 10: NAPE Engineering Standards Correction

Status: complete; C0-C6 and all eight completion gates pass

## Purpose

Correct the current NAPE CLI V2 implementation to conform with the Attestify
Engineering and Rust Test Standards while preserving both the completed V2
lifecycle and every implemented V1 user capability. Current execution replaces
`AssuranceProcedure` and `AssuranceReport` with `VerificationProcedure` and
`VerificationReport` V2; that contract transition does not authorize a
functionality loss.

## Authority

The complete authoritative plan is:

- [NAPE CLI V2 Engineering Standards Correction Plan](../../../../../1-attestify-product-workspace-docs/0-plans/attestify-oci/nape-cli-v2-engineering-standards-correction-plan.md)

The final result is:

- [NAPE CLI V2 Engineering Standards Correction Completion Review](../../../../../1-attestify-product-workspace-docs/0-plans/attestify-oci/nape-cli-v2-engineering-standards-correction-completion-review.md)

The governing read-only review is:

- [NAPE CLI V2 Engineering and Test Standards Code Review](../../../../../1-attestify-product-workspace-docs/0-plans/attestify-oci/nape-cli-v2-engineering-and-test-standards-code-review.md)

The immutable V1 capability audit is:

- [NAPE V1 Capability to Verification V2 Reconciliation Audit](../../../../../1-attestify-product-workspace-docs/0-plans/attestify-oci/nape-v1-capability-to-verification-v2-reconciliation-audit.md)

The current feedback document is:

- [NAPE V1 Capability Parity and Verification V2 Interface Review](../../../../../1-attestify-product-workspace-docs/0-plans/attestify-oci/nape-v1-capability-parity-and-verification-v2-interface-review.md)

The approved authoritative correction within C3 is:

- [NAPE C3 Attestify OCI and NAPE Ownership Correction Plan](../../../../../1-attestify-product-workspace-docs/0-plans/attestify-oci/nape-c3-attestify-oci-and-nape-ownership-correction-plan.md)

This repository-local document is a navigation and restart seam. It does not
duplicate or supersede the central plan.

## Ordered slices

1. C0: characterization, ownership, and parity freeze.
2. C1: Kernel, bounded values, diagnostics, and parser capabilities.
3. C2: six Domain Use Cases and deterministic subordinate services.
4. C3: Application Gateway drivers and client-library correction, governed by
   one reusable `attestify-oci` package/profile owner, thin NAPE drivers, and
   cross-process verified package custody.
5. C4: thin CLI adapters, composition, and cutover.
6. C5: test, module, documentation, and quality conformance.
7. C6: full parity, hostile-input, and red-team closure.

## Completion boundary

All eight gates in the central plan must pass. Stop before the Attestify OCI
Repository Profile `0.1.0` snapshot and before every excluded post-I6 feature.

NAPE V2 has exactly one Procedure-execution lifecycle: Start, repeated
one-file Evidence, and argument-free Verify against the NAPE-managed current
run. Start and Evidence preserve V1's silent success behavior. Verify retains
the governed Command Receipt V2 projection. No Start/Evidence Receipt variant,
caller workspace, run handle, evidence-root Verify input, or alternate
execution mode is part of this correction.

The bounded Product-contract changes are the Verification Report V2 metadata
amendment and its current selector closure. The selector preserves historical
I4/I5 definitions and existing feature-rich I6 semantics while selecting
exactly one whole-Report Evaluator-V2/development-v1 or Evaluator-V3/
development-v2 mode. The current Evidence Set relationship selector exposes
the unchanged I6-capacity relationship.

Planning and interface reconciliation are complete. Continuous C0-through-C6
implementation was authorized on 2026-08-03. C0-C3 are complete. Six public
Domain Use Cases, fourteen NAPE Gateway seams, seven subordinate services, the
reusable `attestify-oci` boundary, and restartable verified package custody are
frozen. C4 composes those seams and cuts the command surface over to the
staged V2 lifecycle. C5 closes module, test, documentation, and non-live
quality conformance. C6 closes Receipt reference-class projection, private
Evaluator workspace custody, current Report/relationship selection, hostile-
input coverage, and two clean-registry qualifications.
