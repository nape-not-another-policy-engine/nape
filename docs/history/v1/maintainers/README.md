# Maintainer Documentation

This folder is for engineers changing or reviewing NAPE implementation.

## Reading Order

1. [Architecture](architecture.md)
2. [Maintainer decisions](decisions/README.md)
3. [Local development](local-development.md)

## What Each Document Covers

`architecture.md`

How CLI commands map to command handlers, domain use cases, gateway adapters, filesystem state, serializers, and `nape-eval`.

`decisions/`

Lightweight architecture decision records for current V1 design choices that matter for V2 planning.

`local-development.md`

Build, test, external dependencies, local smoke tests, and documentation verification.

## Maintainer Source Of Truth

Primary implementation areas:

- `apps/nape-cli/src/main.rs`
- `apps/nape-cli/src/io_adapter/clap/`
- `apps/nape-cli/src/usecase_configuration/`
- `apps/nape-cli/src/gateway_adapter/`
- `apps/nape-cli/src/state_management/`
- `domain/src/evidence_collection/usecases/`

Shared value-object rules come from `kernel_oss` tag `0.2.5`, locked to commit `5410baccae23356f1bb59902e7f7afc452577db2`.

## What Is Not Here

- User walkthroughs live in `../user/`.
- File-format reference lives in `../reference/`.
- Product-level behavior and limitations live in `../product/`.
