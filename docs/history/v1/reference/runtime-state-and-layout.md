# Runtime State And Output Layout

NAPE is filesystem-oriented. It writes an active state file under the user's home directory and writes run outputs under the current working directory.

## Active State File

Path:

```text
$HOME/nape/.nape_cli_config
```

This file points `nape collect evidence` and `nape collect report` at the current run.

Only one run is active at a time. A new `nape collect start` overwrites this state file.

## State File Shape

```yaml
subject_nrn: nrn:procedure:rover-medical/rover-medicine-system
subject_id: localrelease3
procedure_repository: file:///private/tmp/rover-medical-catalog
procedure_directory: rover-mediical-system/release-3
metadata:
  utc-start: '1782835488890'
  system-owner: Bill Bensing
directories:
  home: /private/tmp/nape-rover-example/rover-medical-system/nrn_procedure_rover-medical_-_rover-medicine-system/1782835488890
  assurance-procedure-file: /private/tmp/nape-rover-example/rover-medical-system/nrn_procedure_rover-medical_-_rover-medicine-system/1782835488890/assurance_procedure.yaml
  evidence: /private/tmp/nape-rover-example/rover-medical-system/nrn_procedure_rover-medical_-_rover-medicine-system/1782835488890/evidence
  activity-test: /private/tmp/nape-rover-example/rover-medical-system/nrn_procedure_rover-medical_-_rover-medicine-system/1782835488890/activity
  temp: /private/tmp/nape-rover-example/rover-medical-system/nrn_procedure_rover-medical_-_rover-medicine-system/1782835488890/temp
```

## State Fields

| Field | Description |
| --- | --- |
| `subject_nrn` | Subject NRN from `nape collect start`. |
| `subject_id` | Subject ID from `nape collect start`. |
| `procedure_repository` | Procedure repository link from `nape collect start`. |
| `procedure_directory` | Procedure directory from `nape collect start`. |
| `metadata` | User metadata plus generated `utc-start`. |
| `directories.home` | Run workspace root. |
| `directories.assurance-procedure-file` | Retrieved procedure YAML path. |
| `directories.evidence` | Evidence root directory. |
| `directories.activity-test` | Activity test root directory. |
| `directories.temp` | Temporary retrieval directory used during start. |

## Output Directory Encoding

NAPE creates run output in the current working directory.

The top-level output directory is derived from the subject NRN:

- `:` becomes `_`
- `/` becomes `_-_`

Example:

```text
nrn:procedure:rover-medical/rover-medicine-system
```

becomes:

```text
nrn_procedure_rover-medical_-_rover-medicine-system
```

The run instance directory is the UTC millisecond start time.

Full shape:

```text
<encoded-subject-nrn>/<utc-start>/
```

## Run Workspace Layout

```text
<encoded-subject-nrn>/<utc-start>/
  assurance_procedure.yaml
  assurance_report.yaml
  activity/
    <activity-name>/
      <test-files>
  evidence/
    <activity-name>/
      <evidence-files>
```

`assurance_report.yaml` exists only after `nape collect report` succeeds.

## Evidence Copy Behavior

Given:

```bash
nape collect evidence \
  --control-activity "pet-medicine-db" \
  --file-path "pet-medicine-db/my-db.ini"
```

NAPE copies:

```text
pet-medicine-db/my-db.ini
```

to:

```text
<run-home>/evidence/pet-medicine-db/my-db.ini
```

Given:

```bash
nape collect evidence \
  --control-activity "pet-medicine-db" \
  --file-path "./generated/database.ini" \
  --file-name "my-db.ini"
```

NAPE copies:

```text
./generated/database.ini
```

to:

```text
<run-home>/evidence/pet-medicine-db/my-db.ini
```

## Isolating Runs

For local testing, set `HOME` to a temporary directory before running NAPE:

```bash
WORK=/private/tmp/nape-run
mkdir -p "$WORK/home"
export HOME="$WORK/home"
```

This prevents a demo run from overwriting your normal `$HOME/nape/.nape_cli_config`.
