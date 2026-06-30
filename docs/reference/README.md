# Reference Documentation

This folder describes NAPE file formats and runtime contracts. Use these docs when you need precise structure rather than a walkthrough.

## Documents

- [Assurance procedure format](assurance-procedure-format.md): YAML input consumed from a procedure repository.
- [Assurance report format](assurance-report-format.md): YAML output written by `nape collect report`.
- [V1 contract matrix](v1-contract-matrix.md): schema-like V1 field, runtime, evaluator, and path contracts for V2 planning.
- [V1 source traceability](v1-source-traceability.md): maps V1 behavior and contracts to CLI, domain, adapter, and kernel source files.
- [Evaluation to report traceability](evaluation-report-traceability.md): how procedure actions become evaluator calls and report fields.
- [Runtime state and output layout](runtime-state-and-layout.md): `$HOME/nape/.nape_cli_config`, run workspace layout, and evidence copy behavior.

## Use This Folder When

- Creating or reviewing `assurance_procedure.yaml`
- Reading `assurance_report.yaml`
- Planning V2 procedure or report compatibility
- Tracing documented behavior to implementation source files
- Tracing report fields back to procedure and evaluator sources
- Debugging missing evidence files
- Understanding where NAPE stores active run state
- Verifying the output directory created from a subject NRN

## What Is Not Here

- Step-by-step CLI usage lives in `../user/`.
- Implementation flow lives in `../maintainers/`.
- Product overview and public contracts live in `../product/`.
