# CLI Workflow

NAPE uses a three-step workflow:

1. `nape collect start`
2. `nape collect evidence`
3. `nape collect report`

Each step depends on the previous one.

## Step 1: Start A Collection

```bash
nape collect start \
  --subject "nrn:procedure:rover-medical/rover-medicine-system" \
  --subject-id "localrelease3" \
  --procedure-link "file:///private/tmp/rover-medical-catalog" \
  --procedure-directory "rover-mediical-system/release-3" \
  --meta system-owner "Bill Bensing"
```

This command:

- Validates subject, procedure, and metadata inputs.
- Creates the run directory under the current working directory.
- Copies the assurance procedure and activity tests into the run directory.
- Writes active run state to `$HOME/nape/.nape_cli_config`.

Generated workspace shape:

```text
<encoded-subject-nrn>/<utc-start>/
  assurance_procedure.yaml
  activity/
  evidence/
```

The encoded subject NRN replaces `:` with `_` and `/` with `_-_`.

Example:

```text
nrn:procedure:rover-medical/rover-medicine-system
```

becomes:

```text
nrn_procedure_rover-medical_-_rover-medicine-system
```

## Step 2: Collect Evidence

```bash
nape collect evidence \
  --control-activity "pet-medicine-app" \
  --file-path "pet-medicine-app/app-config.toml"
```

This command:

- Reads `$HOME/nape/.nape_cli_config`.
- Validates `--control-activity` as a NAPE name.
- Reads the source evidence file.
- Copies it into the current run workspace.

Use an activity name for `--control-activity`, not an action name. NAPE copies evidence under an activity directory, and procedure actions reference files in those directories.

Output path:

```text
<run-home>/evidence/<control-activity>/<source-file-name>
```

Use `--file-name` to rename the evidence file as it is copied:

```bash
nape collect evidence \
  --control-activity "pet-medicine-app" \
  --file-path "./tmp/generated-app-config.toml" \
  --file-name "app-config.toml"
```

This is useful when the procedure expects a specific evidence file name.

## Step 3: Generate Report

```bash
nape collect report
```

This command:

- Reads `$HOME/nape/.nape_cli_config`.
- Loads `<run-home>/assurance_procedure.yaml`.
- Resolves procedure test paths and evidence paths from the run workspace.
- Invokes `nape-eval` for each evidence/test pair.
- Calculates SHA256 signatures for test and evidence files.
- Writes `<run-home>/assurance_report.yaml`.

## Active Run State

NAPE tracks the current run in:

```text
$HOME/nape/.nape_cli_config
```

Only one active run is tracked at a time. Running `nape collect start` again overwrites the state file with the new run.

Example state file:

```yaml
subject_nrn: nrn:procedure:rover-medical/rover-medicine-system
subject_id: localrelease3
procedure_repository: file:///private/tmp/rover-medical-catalog
procedure_directory: rover-mediical-system/release-3
metadata:
  utc-start: '1782835488890'
  system-owner: Bill Bensing
directories:
  home: /private/tmp/nape-doc-release3/rover-medical-system/nrn_procedure_rover-medical_-_rover-medicine-system/1782835488890
  assurance-procedure-file: /private/tmp/nape-doc-release3/rover-medical-system/nrn_procedure_rover-medical_-_rover-medicine-system/1782835488890/assurance_procedure.yaml
  evidence: /private/tmp/nape-doc-release3/rover-medical-system/nrn_procedure_rover-medical_-_rover-medicine-system/1782835488890/evidence
  activity-test: /private/tmp/nape-doc-release3/rover-medical-system/nrn_procedure_rover-medical_-_rover-medicine-system/1782835488890/activity
  temp: /private/tmp/nape-doc-release3/rover-medical-system/nrn_procedure_rover-medical_-_rover-medicine-system/1782835488890/temp
```

## Successful Command Output

Current successful commands usually print no terminal output. Confirm success by inspecting files:

```bash
find . -name assurance_procedure.yaml -o -name assurance_report.yaml
find . -path "*/evidence/*" -type f
sed -n '1,120p' "$HOME/nape/.nape_cli_config"
```

## Common Workflow Errors

**Subject ID contains punctuation**

Problem:

```text
--subject-id local-release-3
```

Fix:

```text
--subject-id localrelease3
```

`--subject-id` must be alphanumeric only.

**Procedure link uses SSH**

Problem:

```text
--procedure-link ssh://github.com/attestify/rover-medical-catalog.git
```

Fix:

```text
--procedure-link https://github.com/attestify/rover-medical-catalog.git
```

or, for local clones:

```text
--procedure-link file:///private/tmp/rover-medical-catalog
```

**Metadata omitted**

Problem:

```bash
nape collect start ...
```

without `--meta`.

Fix:

```bash
--meta system-owner "Bill Bensing"
```

Current CLI help implies metadata is optional, but the current handler expects metadata to be present.

**Evidence missing**

If the procedure references `evidence/pet-medicine-db/my-db.ini`, then evidence collection must create:

```text
<run-home>/evidence/pet-medicine-db/my-db.ini
```

Collect it with:

```bash
nape collect evidence \
  --control-activity "pet-medicine-db" \
  --file-path "pet-medicine-db/my-db.ini"
```

Here `pet-medicine-db` is the activity name from the procedure, and `my-db.ini` is the evidence file expected by one or more actions in that activity.

**Evaluator not installed**

`nape collect report` requires `nape-eval`.

Check:

```bash
nape-eval --check-install
```

Install:

```bash
python3 -m pip install nape
```
