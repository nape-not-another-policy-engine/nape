# NAPE CLI 2.0 Quickstart

Build and execute the checked minimal embedded Procedure without a registry.

```bash
cd docs/examples/verification-v2-oci

nape package build \
  --source ./minimal/source \
  --package pkg:attestify/acme.example/verification-procedure/release-readiness/database-endpoint@1.0.0 \
  --output ./build/minimal-procedure

nape start \
  --local-package ./build/minimal-procedure \
  --subject-file ./minimal/subject.json

nape evidence \
  --action release-readiness.database-connection \
  --file ./minimal/evidence/release-readiness/database-connection/application-configuration.json

nape verify
```

Start creates a new NAPE-managed current run, and each Evidence command adds or
replaces one Action occurrence's payload. Start and Evidence success are
silent. Verify atomically creates a Verification Report, an Evidence Set
relationship, and one digest-addressed copy of each unique raw evidence
payload. Verify emits exactly one Receipt V2 JSON line whose `result.output`
identifies that immutable result directory.

Continue with the [complete OCI lifecycle](cli-workflow.md).
