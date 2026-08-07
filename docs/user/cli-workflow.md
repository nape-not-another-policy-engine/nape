# Complete Verification V2 OCI Lifecycle

Use the checked complete example in
[`../examples/verification-v2-oci`](../examples/verification-v2-oci/README.md).
The lifecycle is:

1. Build the independent Action.
2. Build the Activity with the exact Action build result.
3. Build the Procedure with the complete Action and Activity build-result set.
4. Publish those exact bytes without rebuilding.
5. Resolve the root PURL plus manifest digest.
6. Start a NAPE-managed run from the exact OCI root PURL and digest.
7. Add required evidence one file at a time with `nape evidence`.
8. Execute the argument-free `nape verify` against the frozen current run.
9. Inspect the atomically persisted Report, Evidence Set relationship, and raw evidence.

For one registry exposed through a localhost port or port forward, pass:

```text
--registry-endpoint http://localhost:5001
```

The endpoint applies only to the current command. It maps every canonical
`pkg:attestify` publisher in the exact closure to the endpoint under the fixed
`attestify/` repository prefix. It does not alter the PURL or enter package
bytes.

Use `--registry-profile` instead when different publishers require different
exact endpoints or prefixes. NAPE never searches registries or falls back
between the two forms.

Every OCI Start requires the root package PURL and exact manifest digest. Tags
are publication convenience, not execution authority. Verify performs no OCI
operation. OCI Report publication is not part of this workflow.

See [Staged Verification Workflow](staged-verification-workflow.md) for exact
Start, Evidence, and Verify commands.
