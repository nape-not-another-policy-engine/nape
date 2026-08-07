# Staged Verification Workflow

Start one new run from a local build result:

```text
nape start --local-package ./build/procedure --subject-file ./subject.yaml
```

Or start from one exact OCI release:

```text
nape start --package pkg:attestify/acme.example/verification-procedure/release/readiness@1.0.0 \
  --manifest-digest sha256:<64-lowercase-hex> \
  --registry-endpoint http://localhost:5001 \
  --subject-file ./subject.yaml
```

Add evidence one piece at a time. The host source basename may differ. Use `--file-name` only to assert the definition-owned filename.

```text
nape evidence --action release-readiness.database-connection --file ./exports/orders.json
nape evidence --action release-readiness.primary-tls-policy --file ./tls.json
```

Repeating an Action with equal bytes is idempotent. Different bytes atomically replace only that association while the run is collecting.

Execute the current run:

```text
nape verify
```

Verify accepts no source, evidence, workspace, run-handle, or output arguments. Readiness failure leaves the run collecting so missing evidence can be supplied. Success makes the completed output immutable. A subsequent `nape start` creates and selects a distinct run directory.

![Staged lifecycle](../assets/diagrams/plantuml/rendered/09-staged-verification.svg)

Source: [09-staged-verification.puml](../assets/diagrams/plantuml/source/09-staged-verification.puml)
