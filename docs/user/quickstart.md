# NAPE Quickstart

This guide runs the smallest useful local NAPE workflow: start a collection, collect one evidence file, and generate an assurance report.

## Prerequisites

Install or build:

- `nape`
- `nape-eval`

See [Installation](installation.md) for full setup instructions.

Check the tools:

```bash
nape --help
nape-eval --check-install
```

If you are building from this repository:

```bash
cargo build --release -p nape_cli
```

The binary is produced at:

```text
target/release/nape
```

`nape-eval` is a separate evaluator CLI. The Rover GitHub Actions setup installs it with:

```bash
python3 -m pip install nape
```

## Clone Example Repositories

Create a temporary demo workspace:

```bash
WORK=/private/tmp/nape-rover-quickstart
rm -rf "$WORK"
mkdir -p "$WORK"
```

Clone the Rover example repositories.

Using SSH:

```bash
git clone git@github.com:attestify/rover-medical-system.git "$WORK/rover-medical-system"
git clone git@github.com:attestify/rover-medical-catalog.git "$WORK/rover-medical-catalog"
```

Using HTTPS:

```bash
git clone https://github.com/attestify/rover-medical-system.git "$WORK/rover-medical-system"
git clone https://github.com/attestify/rover-medical-catalog.git "$WORK/rover-medical-catalog"
```

NAPE does not currently accept `ssh://` as a `--procedure-link` scheme. Cloning with SSH is fine, but use `file://` when pointing NAPE at the local catalog clone.

This guide was validated against the Rover commits listed in [example outputs](../examples/README.md).

## Isolate NAPE State

NAPE stores active run state in `$HOME/nape/.nape_cli_config`. For a safe demo, isolate `HOME`:

```bash
mkdir -p "$WORK/home"
export HOME="$WORK/home"
```

## Start A Collection

Run from the Rover system repository:

```bash
cd "$WORK/rover-medical-system"

nape collect start \
  --subject "nrn:procedure:rover-medical/rover-medicine-system" \
  --subject-id "localrelease1" \
  --procedure-link "file://$WORK/rover-medical-catalog" \
  --procedure-directory "rover-mediical-system/release-1" \
  --meta system-owner "Bill Bensing"
```

Notes:

- `localrelease1` is alphanumeric because `--subject-id` cannot contain dashes or underscores.
- `rover-mediical-system` is intentionally spelled the way the current catalog path is spelled.
- Include at least one `--meta` value until the current optional-metadata bug is fixed.

On success, this command usually prints nothing.

## Collect Evidence

Release 1 uses the app config already present in the Rover system repository:

```bash
nape collect evidence \
  --control-activity "pet-medicine-app" \
  --file-path "pet-medicine-app/app-config.toml"
```

On success, this command usually prints nothing.

## Generate The Report

```bash
nape collect report
```

On success, this command usually prints nothing.

## Inspect Outputs

List the generated report:

```bash
find . -name assurance_report.yaml -print
```

Typical output path:

```text
./nrn_procedure_rover-medical_-_rover-medicine-system/<utc-start>/assurance_report.yaml
```

Inspect the summary:

```bash
sed -n '1,80p' nrn_procedure_rover-medical_-_rover-medicine-system/*/assurance_report.yaml
```

For the current Rover Release 1 thin-slice example, expect:

```yaml
summary:
  activity_count: 1
  action_count: 1
  actions_run: 1
  pass: 0
  fail: 0
  inconclusive: 1
  outcome: inconclusive
```

## What Happened

`nape collect start` created a run workspace under the current directory and wrote state to:

```text
$HOME/nape/.nape_cli_config
```

`nape collect evidence` copied:

```text
pet-medicine-app/app-config.toml
```

to:

```text
<run-home>/evidence/pet-medicine-app/app-config.toml
```

`nape collect report` invoked `nape-eval`, evaluated the procedure action, calculated file signatures, and wrote:

```text
<run-home>/assurance_report.yaml
```
