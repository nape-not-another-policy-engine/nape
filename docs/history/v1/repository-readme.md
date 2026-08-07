# NAPE

NAPE is a command-line tool for collecting evidence, applying tests of detail, and generating assurance reports.

It is built for assurance workflows where a team needs to prove what a system says it should be doing against evidence of what it is actually doing. NAPE does this by retrieving an assurance procedure, copying evidence into a local run workspace, evaluating evidence with procedure-defined tests, and writing an assurance report.

## How It Works

The current CLI workflow has three steps:

```bash
nape collect start
nape collect evidence
nape collect report
```

`nape collect start` creates a local run workspace from an assurance procedure repository.

`nape collect evidence` copies evidence files into that workspace under an activity name.

`nape collect report` invokes `nape-eval`, evaluates the collected evidence, signs the evaluated files, and writes `assurance_report.yaml`.

## Core Concepts

- `Subject`: the system, repository, procedure, or resource being evaluated.
- `Assurance Procedure`: a YAML file that defines activities, actions, test files, and expected evidence paths.
- `Activity`: a named group of related actions. Evidence is collected under an activity directory.
- `Action`: one testable expectation inside an activity.
- `Evidence`: a file copied into the NAPE run workspace.
- `Test of Detail`: a Python evaluator file invoked through `nape-eval`.
- `Assurance Report`: the generated YAML result with outcomes, reasons, summary counts, and SHA256 signatures.

## Repository Layout

```text
apps/nape-cli/   CLI parsing, command handlers, adapters, serializers, evaluator integration
domain/          Evidence collection use cases and use case boundaries
docs/            Product, user, reference, and maintainer documentation
```

## Documentation

Start by audience:

- New CLI users: [Quickstart](docs/user/quickstart.md), then [CLI workflow](docs/user/cli-workflow.md)
- Users running the demo: [Rover Medical example](docs/user/rover-medical-example.md)
- Users installing locally: [Installation](docs/user/installation.md)
- Users authoring procedures: [Procedure authoring](docs/user/procedure-authoring.md)
- Users checking commands: [CLI reference](docs/user/cli-reference.md)
- Maintainers: [Architecture](docs/maintainers/architecture.md), then [Local development](docs/maintainers/local-development.md)
- Format readers: [V1 contract matrix](docs/reference/v1-contract-matrix.md), [V1 source traceability](docs/reference/v1-source-traceability.md), [Assurance procedure format](docs/reference/assurance-procedure-format.md), [Assurance report format](docs/reference/assurance-report-format.md), [Evaluation to report traceability](docs/reference/evaluation-report-traceability.md), and [Runtime state and output layout](docs/reference/runtime-state-and-layout.md)
- Output examples: [Example outputs](docs/examples/README.md)

Documentation index:

- [Docs overview](docs/README.md)

Product:

- [Product specification](docs/product/nape-product-spec.md)
- [V1 verification baseline](docs/product/v1-verification-baseline.md)

Reference:

- [V1 contract matrix](docs/reference/v1-contract-matrix.md)
- [V1 source traceability](docs/reference/v1-source-traceability.md)
- [Assurance procedure format](docs/reference/assurance-procedure-format.md)
- [Assurance report format](docs/reference/assurance-report-format.md)
- [Evaluation to report traceability](docs/reference/evaluation-report-traceability.md)
- [Runtime state and output layout](docs/reference/runtime-state-and-layout.md)

Maintainers:

- [Architecture](docs/maintainers/architecture.md)
- [Local development](docs/maintainers/local-development.md)

## Current Limitations

- `--meta` is described as optional by CLI help, but the current `collect start` implementation expects at least one metadata pair.
- `--procedure-link` accepts `file`, `git`, and `https`; it does not accept `ssh://`.
- The CLI currently writes local reports and does not upload results to a repository, despite older help text mentioning upload behavior.
- NAPE tracks one active run in `$HOME/nape/.nape_cli_config`.
- Successful commands usually print no terminal output; inspect generated files to confirm success.

## Build

```bash
cargo build
```

Build the CLI release binary:

```bash
cargo build --release -p nape_cli
```

## Test

```bash
cargo test
```

Package-specific targets:

```bash
make test-nape-cli
make test-domain
```
