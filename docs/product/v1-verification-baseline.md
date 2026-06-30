# V1 Verification Baseline

This document is the permanent V1 baseline for planning V2 verification procedures and verification reports.

V2 should preserve V1 compatibility where practical. Breaking changes are allowed when they produce a clearer, safer, or more durable verification model, but each breaking change should be explicit and justified.

## Scope

This baseline covers the current implemented verification flow:

1. A user starts a run from an assurance procedure repository.
2. A user collects local evidence files under activity names.
3. NAPE evaluates procedure actions with `nape-eval`.
4. NAPE writes a local `assurance_report.yaml`.

This baseline does not define V2 behavior. It records V1 behavior so V2 can be designed against a known current state.

## Source Of Truth

Current V1 behavior is derived from:

- `apps/nape-cli`
- `domain`
- `kernel_oss` tag `0.2.5`, locked to commit `5410baccae23356f1bb59902e7f7afc452577db2`
- Rover Medical example repositories listed in `docs/examples/README.md`

Related permanent docs:

- `docs/reference/v1-contract-matrix.md`
- `docs/reference/v1-source-traceability.md`
- `docs/reference/assurance-procedure-format.md`
- `docs/reference/assurance-report-format.md`
- `docs/reference/evaluation-report-traceability.md`
- `docs/reference/runtime-state-and-layout.md`
- `docs/user/procedure-authoring.md`

## Current User Workflow

The current CLI workflow has three commands:

```bash
nape collect start
nape collect evidence
nape collect report
```

`nape collect start` retrieves a procedure directory, creates a local run workspace, and writes active run state.

`nape collect evidence` copies a local evidence file into the active run workspace under an activity directory.

`nape collect report` loads the procedure, evaluates action evidence/test pairs with `nape-eval`, signs evidence and test files, and writes `assurance_report.yaml`.

## Installation Baseline

The official public installation path is the NAPE binary repository for the NAPE CLI.

The repository-local installation guide remains focused on current checkout and maintainer workflows. Source build instructions are developer fallback instructions, not the primary public install path.

Writerside under `../docs/Writerside` is the public documentation source for polished installation flows. This repository keeps current-behavior and developer-local installation documentation.

Binary release integrity verification is a V2/release-process follow-up. Public binary releases should publish checksum, signature, or provenance artifacts before the docs present verification commands. Until that convention exists, the repository installation guide includes an integrity caveat instead of inventing unsupported commands.

## V1 Procedure Baseline

A V1 procedure directory contains:

```text
assurance_procedure.yaml
activity/
```

The current procedure YAML model contains:

- `apiVersion`
- `kind`
- `procedure`
- `activity[]`
- `activity[].action[]`

Each action points to one test path and one evidence path. Multiple actions may point to the same evidence path.

`expect-evidence` may appear in older examples, but it is not part of the current V1 serializer contract used by this repository.

## V1 Report Baseline

`nape collect report` writes:

```text
<run-home>/assurance_report.yaml
```

The current report model contains:

- `apiVersion`
- `kind`
- `metadata`
- `subject`
- `procedure`
- `summary`
- `activity[]`
- `activity[].action[]`

Each report action contains:

- action name
- evaluator outcome
- evaluator reason
- signed test file reference
- signed evidence file reference

Signatures are SHA256 signatures over the file bytes read from the run workspace. The report stores the procedure-relative file path, not the canonical filesystem path used for reading bytes.

## Outcome Baseline

Valid action outcomes are:

- `pass`
- `fail`
- `inconclusive`
- `error`

The current summary outcome behavior is:

- If any action is `inconclusive` or `error`, the report outcome is `inconclusive`.
- Otherwise, if any action is `fail`, the report outcome is `fail`.
- Otherwise, the report outcome is `pass`.

The current report summary does not expose a separate `error` count.

## Path Baseline

Procedure action paths are intended to be relative to the run workspace:

```yaml
test: "activity/pet-medicine-app/database-connection.py"
evidence: "evidence/pet-medicine-app/app-config.toml"
```

Current behavior combines the run home path with the procedure action path before evaluating or signing files. V1 docs should describe paths as workspace-relative and should avoid recommending absolute paths, `../`, or paths outside the run workspace.

V2 should explicitly decide path grammar, normalization, and rejection behavior.

## Current V1 Debt Disposition

| Item | Current V1 behavior | V1 disposition | V2 disposition |
| --- | --- | --- | --- |
| CLI help mentions upload | Help text implies repository upload, but implementation writes local reports. | Document as current mismatch. | Fix in V2 CLI help or implement upload explicitly. |
| Control action/activity wording | CLI help says Control Action, flag and behavior use activity. | Document activity behavior. | Standardize user-facing and internal terminology. |
| `--meta` optionality mismatch | CLI help presents metadata as optional, handler expects at least one occurrence. | Document as current limitation. | Decide whether metadata is optional and update code/help/contracts. |
| `ssh://` procedure links | Kernel repository link contract rejects `ssh://`. | Document local clone plus `file://` workaround. | Decide whether SSH repository links are supported. |
| Single active state file | `$HOME/nape/.nape_cli_config` points to one active run. | Document isolated `HOME` workaround. | Decide whether multi-run state is required. |
| No terminal output on success | Successful commands usually print no output. | Document file-inspection confirmation. | Consider success summaries for V2 UX. |
| No `error` summary count | `error` outcomes are possible but not separately counted. | Document summary shape. | Decide whether V2 report needs explicit error accounting. |
| Path semantics | Workspace-relative intent, limited explicit grammar. | Document safe path conventions. | Define path grammar, normalization, and rejection rules. |
| Binary install integrity | Official binary install path exists, but checksum/signature/provenance artifacts are not yet documented as release outputs. | Document caveat; do not block this docs commit. | Define release artifact convention, update Writerside install verification, and add release checklist coverage. |
| Rover Release 4 drift | Demo narrative mentions drift, inspected evidence/tests do not prove distinct drift. | Document caveat. | Fix demo evidence/tests or remove drift claim from V2 examples. |

## V2 Design Inputs

The V2 implementation plan should explicitly answer:

- Which V1 procedure fields remain compatible?
- Which V1 report fields remain compatible?
- Is `apiVersion: 1.0.0` accepted by V2?
- Does V2 produce a new report `apiVersion`?
- Are V1 reports still readable by V2 tools?
- Are V1 procedures still executable by V2 tools?
- Are action outcomes still limited to `pass`, `fail`, `inconclusive`, and `error`?
- Does V2 preserve local filesystem reports, add upload behavior, or both?
- Does V2 preserve one active run per `HOME`, or introduce explicit run IDs?
- Does V2 treat command failures as reportable outcomes, fatal errors, or both?
- Does V2 introduce machine-readable schemas for procedures and reports?
- What binary-release verification artifacts are required before public install docs show verification commands?
- Does the release process publish checksums only, signed checksum manifests, or stronger provenance/attestation artifacts?

## Recommended V2 Compatibility Position

Default position:

- Preserve V1 input compatibility when the current behavior is clear and safe.
- Prefer additive report fields over removing V1 fields.
- Allow breaking changes when V1 behavior is ambiguous, unsafe, or blocks a clearer V2 contract.
- Record every breaking change with rationale and migration guidance.
