# NAPE Documentation

This folder is the documentation map for NAPE. Use it to choose the right path based on whether you are using the CLI, inspecting file formats, or maintaining the code.

## Start Here

- New to NAPE: read the [product specification](product/nape-product-spec.md), then the [quickstart](user/quickstart.md).
- Running the CLI: read [installation](user/installation.md), [CLI workflow](user/cli-workflow.md), and [CLI reference](user/cli-reference.md).
- Authoring procedures: read [procedure authoring](user/procedure-authoring.md), then the [assurance procedure format](reference/assurance-procedure-format.md).
- Running the Rover demo: read [Rover Medical example](user/rover-medical-example.md).
- Understanding files: read [V1 contract matrix](reference/v1-contract-matrix.md), [V1 source traceability](reference/v1-source-traceability.md), [assurance procedure format](reference/assurance-procedure-format.md), [assurance report format](reference/assurance-report-format.md), [evaluation to report traceability](reference/evaluation-report-traceability.md), and [runtime state and output layout](reference/runtime-state-and-layout.md).
- Maintaining code: read [architecture](maintainers/architecture.md), then [local development](maintainers/local-development.md).
- Planning and handoff readers: start with [roadmap](1-plan/roadmap.md).
- Comparing outputs: inspect [example outputs](examples/README.md).

## Folder Guide

`product/`

Product-level behavior, users, capabilities, contracts, and current limitations.

Start with `product/nape-product-spec.md`, then use `product/v1-verification-baseline.md` when planning V2 verification procedure or report changes.

`user/`

Task-oriented CLI documentation for installing, running, troubleshooting, and using examples.

`reference/`

Stable reference material for file formats, runtime state, workspace layout, and output contracts.

`maintainers/`

Implementation-oriented docs for engineers changing the CLI, domain use cases, adapters, serializers, or docs.

Maintainer decision records live under `maintainers/decisions/`.

`examples/`

Checked-in example outputs from validated local Rover Medical runs.

`1-plan/`

`nape`-specific roadmap, plan, and handoff material.

## Current Source Of Truth

The docs describe current behavior from:

- `apps/nape-cli`
- `domain`
- `kernel_oss` tag `0.2.5`, locked to commit `5410baccae23356f1bb59902e7f7afc452577db2`
- Rover Medical example repositories listed in [Rover Medical example](user/rover-medical-example.md)

## Planning Rule

Keep `nape` planning and handoff documents under `docs/1-plan/`.
Do not create new `TEMP-` planning files at the root of `docs/`.
