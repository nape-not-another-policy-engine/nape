# CLI Reference

## `nape`

```text
Usage: nape [COMMAND]
```

Commands:

- `collect`
- `help`

## `nape collect`

```text
Usage: nape collect [COMMAND]
```

Commands:

- `start`
- `evidence`
- `report`

Current help text may mention uploading results to a repository. Current implemented behavior writes local run output, including `assurance_report.yaml`.

## `nape collect start`

Starts a new evidence collection procedure by retrieving a NAPE assurance procedure and creating local directories for evidence collection.

```bash
nape collect start \
  --subject "nrn:procedure:rover-medical/rover-medicine-system" \
  --subject-id "localrelease3" \
  --procedure-link "file:///private/tmp/rover-medical-catalog" \
  --procedure-directory "rover-mediical-system/release-3" \
  --meta system-owner "Bill Bensing"
```

Arguments:

| Argument | Short | Required | Description |
| --- | --- | --- | --- |
| `--subject` | `-s` | Yes | NAPE Resource Name for the subject being evaluated. |
| `--subject-id` | `-i` | Yes | Alphanumeric identifier for this subject/run. |
| `--procedure-link` | `-l` | Yes | Procedure repository URL. Supported schemes are `file`, `git`, and `https`. |
| `--procedure-directory` | `-d` | Yes | Directory inside the procedure repository. |
| `--meta` | `-m` | Practically yes | Metadata key/value pair. Current handler expects at least one. |

Validation:

- `--subject` must be a valid NRN.
- `--subject-id` must be non-empty, alphanumeric, and at most 256 characters.
- `--procedure-link` supports `file`, `git`, and `https`.
- `--procedure-directory` cannot contain spaces or contiguous slashes.
- `--meta` keys allow alphanumerics and dashes, with a max length of 64.
- `--meta` values are non-empty after trimming, with a max length of 256.

Outputs:

- Creates `<encoded-subject-nrn>/<utc-start>/`.
- Writes `$HOME/nape/.nape_cli_config`.
- Copies `assurance_procedure.yaml` and `activity/` from the procedure repository into the run workspace.

## `nape collect evidence`

Collects one evidence file and associates it with a control activity.

Important: pass an activity name, not an action name. The file is copied under `<run-home>/evidence/<control-activity>/`, and actions in the assurance procedure may reference evidence paths under that activity directory.

```bash
nape collect evidence \
  --control-activity "pet-medicine-app" \
  --file-path "pet-medicine-app/app-config.toml"
```

Arguments:

| Argument | Short | Required | Description |
| --- | --- | --- | --- |
| `--control-activity` | `-a` | Yes | Activity name to associate with the evidence. |
| `--file-path` | `-f` | Yes | Local source evidence file path. |
| `--file-name` | `-n` | No | Optional target file name to use when copying evidence. |

Validation:

- `--control-activity` must be a NAPE name: alphanumeric and dashes only, no leading or trailing dash.
- `--file-path` must point to a readable local file.
- `--file-name` is not validated by a kernel value object in the current command path; use procedure-expected file names.

Outputs:

```text
<run-home>/evidence/<control-activity>/<file-name>
```

If `--file-name` is omitted, NAPE keeps the source file name.

## `nape collect report`

Evaluates all collected evidence and generates an assurance report.

```bash
nape collect report
```

Arguments:

None.

Dependencies:

- Active state file at `$HOME/nape/.nape_cli_config`
- Retrieved procedure at `<run-home>/assurance_procedure.yaml`
- Collected evidence files under `<run-home>/evidence/`
- Activity test files under `<run-home>/activity/`
- `nape-eval` available on `PATH`

Outputs:

```text
<run-home>/assurance_report.yaml
```

## Valid Input Examples

```bash
--subject "nrn:procedure:rover-medical/rover-medicine-system"
--subject-id "localrelease3"
--procedure-link "file:///private/tmp/rover-medical-catalog"
--procedure-directory "rover-mediical-system/release-3"
--control-activity "pet-medicine-app"
--meta system-owner "Bill Bensing"
```

## Invalid Input Examples

```bash
--subject-id "local-release-3"
--procedure-link "ssh://github.com/attestify/rover-medical-catalog.git"
--procedure-directory "/rover-mediical-system/release-3"
--control-activity "pet_medicine_app"
--meta "system owner" "Bill Bensing"
```

## Outcome Values

Generated reports use these action outcome values:

- `pass`
- `fail`
- `inconclusive`
- `error`

The evaluator accepts outcome values case-insensitively, but reports serialize lowercase values.
