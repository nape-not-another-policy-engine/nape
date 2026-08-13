# NAPE CLI 2.0

NAPE is the Attestify Verification Engine command-line orchestrator. It builds
deterministic Verification V2 definition packages, publishes and pulls exact
packages through OCI, executes every effective Action occurrence through NAPE
Evaluator, and atomically persists the complete local Verification result.

The CLI has two command groups:

```text
nape package
nape verify
```

The former V1 `nape collect` workflow is not part of NAPE CLI 2.0. Its
unchanged documentation is retained under [`docs/history/v1`](docs/history/v1/README.md).

## Start here

- [Quickstart](docs/user/quickstart.md)
- [Complete lifecycle](docs/user/cli-workflow.md)
- [CLI reference](docs/user/cli-reference.md)
- [Procedure authoring](docs/user/procedure-authoring.md)
- [Checked OCI examples](docs/examples/verification-v2-oci/README.md)
- [Architecture](docs/maintainers/architecture.md)

## Current lifecycle

```text
definition source
  -> nape package build
  -> exact local package result
  -> nape package publish
  -> OCI registry
  -> nape package resolve / nape verify
  -> exact closure verification
  -> evidence admission
  -> one Evaluator call per effective Action occurrence
  -> atomic local result directory
```

`pkg:attestify` packages can use either:

- `--registry-endpoint http://localhost:<port>` for one registry endpoint and
  the fixed `attestify/` repository prefix; or
- `--registry-profile <registry-map.yaml>` for exact per-publisher mappings.

The two inputs are mutually exclusive. Authentication, trust evaluation, OCI
Report publication, Risk Engine execution, and enterprise Runner hardening are
outside this development-qualified delivery.

## Output

Successful `nape verify` commits one new directory:

```text
verification-result/
├── verification-report.json
├── evidence-set-relationship.json
└── evidence/sha256/<digest>
```

The result is available locally. NAPE does not publish it to OCI.

## Build and test

```bash
cargo build --workspace
cargo test --workspace
```

NAPE CLI is Rust 2.0.0. Verification definitions and Reports use
`apiVersion: 2.0.0`. Command Receipt V2 and the internal Evaluator V2/V3
boundaries are independently versioned contracts.
