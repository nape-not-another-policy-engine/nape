# Local Development

## Required checks

```bash
make standards-check
```

This warning-unsuppressed command runs formatting, all-target compilation,
strict Clippy, strict Rustdoc, unit and integration tests, structural and test
form checks, documentation/link/diagram checks, and every current generated
Product projection verifier. The separate `attestify-oci-oss` release suite
owns its restricted transport conformance tests. NAPE's release-migration
qualification additionally runs those tests from the exact dependency source
with permission to bind bounded ephemeral loopback ports.

Validate the Product Specification from its repository with the managed Python
environment that provides JSON Schema 2020-12:

```bash
python scripts/validate-verification-engine-i6-contracts.py
```

## Current example gate

The `i2_commands` integration suite builds the checked minimal source twice,
proves byte equality, and builds the complete Action/Activity/Procedure
closure with its exact two-node/two-edge Lock.

## Explicit real Registry and Evaluator gate

Use one anonymous OCI registry exposed at a free localhost port. Pass the exact
endpoint with `--registry-endpoint`; do not assume port 5000. Build every
package twice, publish the exact accepted results, resolve by root PURL and
manifest digest, execute through the real Evaluator, validate outputs, then
repeat from a clean output directory.

Do not add credentials, trust behavior, Report publication, or Fact Finding to
that gate.

```bash
make qualification-live REGISTRY_ENDPOINT=http://localhost:<forwarded-port>
```

The live command is intentionally separate from `make standards-check`; it
requires a clean anonymous development Registry and the real Evaluator
repository. It must be run for C6 and release closure.
