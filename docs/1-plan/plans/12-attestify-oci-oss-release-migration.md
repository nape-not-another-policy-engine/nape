# Plan 12: Attestify OCI OSS Release Migration

Status: **complete; exact-release migration and post-removal gates passed**

## Authority

The binding authority is Workflow Harness
[record 208](../../../../../1-attestify-product-workspace-docs/0-plans/attestify-workflow-harness/harness-specification/workflow-definition-authoring/208-wpc-m1-oss-release-and-nape-migration-material-stop-checkpoint.md)
and the user's continuous OSS-first migration authorization.

## Goal

Replace NAPE's superseded embedded OCI implementation with exact released
`attestify-oci-oss` `0.1.0` while retaining every Verification-specific
profile and adapter in NAPE and proving complete behavioral compatibility.

## Required order

1. Prove the complete NAPE suite against the exact released dependency before
   source removal.
2. Remove only the embedded OCI library, patched OCI-client source, obsolete
   Workspace membership, and qualification helpers tied to those paths.
3. Preserve NAPE's Verification profile, adapters, commands, translations,
   diagnostics, fixtures, and Kernel error mapping.
4. Regenerate the lock and prove exact tag-to-commit resolution.
5. Prove that no embedded or path-based OCI dependency remains.
6. Rerun every focused, Workspace, integration, canonical, standards,
   documentation, and compatibility gate from a clean composite checkout.
7. Commit and push the complete NAPE migration only after every gate passes.

## Exact identities

- OSS release: signed tag `0.1.0`
- OSS commit: `013bb352366fcad80db682984705911ca2108258`
- preserved pre-removal NAPE snapshot:
  `22826ab396361822e7339b3e3a497ef37d26a002`
- final clean composite proof snapshot:
  `b4a672ea733af2f6f166eba1bab0b726ab43bcd1`

## Completion

- The complete exact-release pre-removal suite passed in an environment
  permitted to open its bounded temporary localhost listeners.
- The superseded embedded `libraries/attestify-oci` and patched
  `vendor/oci-client-0.17.0` sources were removed only after that proof.
- The Workspace and lock now resolve only the exact released
  `attestify-oci-oss` `0.1.0` dependency at commit
  `013bb352366fcad80db682984705911ca2108258`.
- NAPE retains every Verification-specific profile, adapter, command,
  diagnostic, fixture, and Kernel error translation.
- The complete post-removal in-place suite and independent clean composite
  suite passed, including formatting, compilation, Clippy, rustdoc, all Rust
  and lifecycle tests, Engineering Standards, documentation, and Product
  Specification projection checks.
- No proprietary OCI implementation, registry operation, or Workflow package
  and catalog behavior was introduced.

## Stop conditions

Stop for semantic, canonical-byte, transport, ownership, licensing, security,
disclosure, panic-policy, or complete-suite divergence. Do not weaken, skip,
or substitute a test. Do not modify `attestify-oci-proprietary` or begin
Workflow package/catalog OCI implementation.
