# NAPE Product Specification

## Summary

NAPE is a command-line tool for running an evidence collection procedure against a subject, collecting evidence files, evaluating those files with tests of detail, and producing an assurance report.

The current product surface is the `nape` CLI. The primary user flow is:

1. Start an evidence collection run from an assurance procedure repository.
2. Collect evidence files and associate them with named control activities.
3. Evaluate the collected evidence and generate an `assurance_report.yaml`.

This document describes current behavior as implemented in this repository and in the locked `kernel_oss` dependency used by this repository.

## Source Of Truth

The current behavior is derived from these sources:

- NAPE CLI crate: `apps/nape-cli`
- NAPE domain crate: `domain`
- Kernel dependency: `kernel_oss` tag `0.2.5`, locked to commit `5410baccae23356f1bb59902e7f7afc452577db2`
- Rover Medical demo repositories:
  - `attestify/rover-medical-system`
  - `attestify/rover-medical-catalog`
  - `attestify/rover-medical-evidence-collection`

## Users

The first documentation audience is CLI users. These users need to know how to install NAPE tooling, run an evidence collection workflow, provide valid inputs, inspect generated outputs, and diagnose common failures.

The second audience is maintainers. Maintainers need to understand how CLI commands map into use cases, gateway adapters, serializers, filesystem state, and the external evaluator.

## Core Concepts

**Subject**

The thing being evaluated. A subject has:

- A NAPE Resource Name, passed with `--subject`
- A subject identifier, passed with `--subject-id`

**Assurance Procedure**

A YAML definition that describes activities, actions, tests, and expected evidence file paths. It lives in a procedure repository and is retrieved during `nape collect start`.

**Activity**

A named group of actions in an assurance procedure. CLI evidence collection associates evidence with an activity by passing `--control-activity`.

**Action**

A testable expectation inside an activity. Each action points to one test file and one evidence file.

**Evidence**

A user-supplied file copied into the NAPE run workspace under `evidence/<activity-name>/`.

**Test Of Detail**

A Python evaluator file referenced by the assurance procedure. NAPE invokes `nape-eval` with the evidence file and test file.

**Assurance Report**

The generated YAML output. It contains subject metadata, procedure metadata, summary counts, action outcomes, reasons, and SHA256 signatures for test and evidence files.

## CLI Capabilities

### `nape collect start`

Starts a new evidence collection run.

Inputs:

- `--subject`
- `--subject-id`
- `--procedure-link`
- `--procedure-directory`
- `--meta`, one or more metadata key/value pairs

Behavior:

- Validates the subject, procedure link, procedure directory, metadata, and API version.
- Creates a run workspace in the current working directory.
- Retrieves `assurance_procedure.yaml` and the `activity/` directory from the procedure repository.
- Writes CLI run state to `$HOME/nape/.nape_cli_config`.

### `nape collect evidence`

Copies one evidence file into the current NAPE run workspace.

Inputs:

- `--control-activity`
- `--file-path`
- Optional `--file-name`

Behavior:

- Reads the current run state from `$HOME/nape/.nape_cli_config`.
- Validates the activity name as a NAPE name.
- Copies the source evidence file to `<run-home>/evidence/<control-activity>/`.
- Uses the original source file name unless `--file-name` is provided.

### `nape collect report`

Evaluates collected evidence and writes the assurance report.

Behavior:

- Reads current run state from `$HOME/nape/.nape_cli_config`.
- Loads the retrieved assurance procedure.
- Resolves each action's test and evidence paths relative to the run workspace.
- Invokes `nape-eval` for each evidence/test pair.
- Calculates SHA256 signatures for evidence and test files.
- Writes `<run-home>/assurance_report.yaml`.

## Input Contracts

### Subject NRN

`--subject` must be an NRN in the form:

```text
nrn:<nid>:<nss>
```

Current allowed NIDs:

- `procedure`
- `sourcecode`

NSS values must be ASCII and cannot contain whitespace.

Examples:

```text
nrn:procedure:rover-medical/rover-medicine-system
nrn:sourcecode:nape/nape-cli
```

### Subject ID

`--subject-id` must be:

- Non-empty
- At most 256 characters
- Alphanumeric only

Valid:

```text
localrelease3
123456789
build20260630
```

Invalid:

```text
local-release-3
release_3
build 123
```

### Procedure Link

`--procedure-link` accepts the current procedure repository schemes:

- `file`
- `git`
- `https`

If no scheme is supplied, the value defaults to `git://`.

Important: `ssh://` is not currently accepted by the procedure link value contract. Users may clone a repository with SSH, but a local NAPE run should use a `file://` procedure link for that clone.

Examples:

```text
https://github.com/attestify/rover-medical-catalog.git
file:///private/tmp/rover-medical-catalog
github.com/example/procedure-catalog
```

### Procedure Directory

`--procedure-directory` identifies the directory inside the procedure repository.

Rules:

- Empty string and `/` are accepted as root.
- Non-root paths cannot start or end with `/`.
- Paths cannot contain contiguous slashes.
- Paths may contain alphanumeric characters, underscores, dashes, and `/`.

Valid:

```text
rover-mediical-system/release-3
rust_ci/sourcecode_integration
```

Invalid:

```text
/rover-mediical-system/release-3
rover-mediical-system/release-3/
rover-mediical-system//release-3
rover medical/release-3
```

### Control Activity

`--control-activity` must be a NAPE name:

- Non-empty
- Alphanumeric and dashes only
- Cannot start or end with a dash

NAPE lowercases the value and uses it as an evidence subdirectory name.

This value is an activity name, not an action name. The current command copies evidence to an activity directory; actions then reference evidence files under that directory.

Valid:

```text
pet-medicine-app
rover-cloud
exa-doo-dc
```

Invalid:

```text
pet_medicine_app
pet medicine app
-pet-medicine-app
pet-medicine-app-
```

### Metadata

`--meta` accepts key/value pairs.

Metadata key rules:

- NAPE name rules
- At most 64 characters

Metadata value rules:

- Trimmed description
- Non-empty after trimming
- At most 256 characters

Example:

```bash
nape collect start \
  --subject "nrn:procedure:rover-medical/rover-medicine-system" \
  --subject-id "localrelease3" \
  --procedure-link "file:///private/tmp/rover-medical-catalog" \
  --procedure-directory "rover-mediical-system/release-3" \
  --meta system-owner "Bill Bensing"
```

Current limitation: CLI help presents `--meta` as optional, but the current command handler unwraps metadata occurrences. Until that bug is fixed, examples should include at least one `--meta` pair.

## Procedure Repository Contract

A procedure directory must contain:

```text
assurance_procedure.yaml
activity/
```

NAPE retrieves the procedure directory and moves:

- `assurance_procedure.yaml` into the run workspace root
- `activity/` into the run workspace root

The procedure's action test paths and evidence paths are interpreted relative to the run workspace.

## Runtime State Contract

NAPE stores current run state at:

```text
$HOME/nape/.nape_cli_config
```

The state file contains:

- `subject_nrn`
- `subject_id`
- `procedure_repository`
- `procedure_directory`
- `metadata`
- `directories`

The `directories` map includes:

- `home`
- `evidence`
- `activity-test`
- `temp`
- `assurance-procedure-file`

`nape collect evidence` and `nape collect report` operate on whichever run is currently recorded in this state file.

## Output Workspace Contract

NAPE creates run output in the current working directory.

The top-level run directory is based on the subject NRN:

- `:` becomes `_`
- `/` becomes `_-_`

The run instance directory is the UTC millisecond start time.

Example:

```text
nrn:procedure:rover-medical/rover-medicine-system
```

becomes:

```text
nrn_procedure_rover-medical_-_rover-medicine-system/<utc-start>/
```

Typical layout:

```text
<encoded-subject-nrn>/<utc-start>/
  assurance_procedure.yaml
  assurance_report.yaml
  activity/
  evidence/
```

## External Dependencies

NAPE CLI depends on:

- `nape`, the Rust CLI built by this repository or installed as a binary
- `nape-eval`, the evaluator CLI invoked during `nape collect report`
- A procedure repository containing `assurance_procedure.yaml` and `activity/`
- Evidence files present on the local filesystem before `nape collect evidence`

The Rover GitHub Actions example also uses Ansible to collect simulated evidence, but Ansible is not required to run NAPE itself if evidence files already exist.

## Report Contract

`nape collect report` writes:

```text
<run-home>/assurance_report.yaml
```

The report includes:

- `apiVersion`
- `kind`
- `metadata`
- `subject`
- `procedure`
- `summary`
- `activity`

Valid action outcomes are:

- `pass`
- `fail`
- `inconclusive`
- `error`

The summary outcome is derived from action outcomes.

## Current Known Limitations

- `--meta` is described as optional, but `collect start` currently fails if no metadata is supplied.
- `--procedure-link` does not support `ssh://`, even though users may clone example repositories over SSH.
- CLI help text may mention uploading results to a repository, but current implemented behavior writes local run output.
- `nape collect start`, `nape collect evidence`, and `nape collect report` usually produce no output on success.
- Only one active run is tracked in `$HOME/nape/.nape_cli_config`.
- Rover Release 4 is described as a drift demo, but the inspected Release 3 and Release 4 evidence files appeared identical, and the relevant SOC1/cloud tests currently return `inconclusive`.
