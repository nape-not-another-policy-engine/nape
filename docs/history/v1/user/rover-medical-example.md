# Rover Medical Example

The Rover Medical repositories demonstrate NAPE against a simulated medical system release workflow.

This guide runs the example locally without GitHub Actions. It uses the procedure catalog through a local `file://` URL and copies canned evidence files directly from the evidence collection repository.

## Repositories

The example uses three repositories:

- `attestify/rover-medical-system`: demo system and GitHub Actions workflow
- `attestify/rover-medical-catalog`: release-specific NAPE procedures and test files
- `attestify/rover-medical-evidence-collection`: Ansible playbooks and canned evidence files

The `rover-medical-system` workflow normally uses Ansible to collect evidence from the evidence collection repo. For local NAPE documentation and validation, copying the canned evidence into the system repo is enough.

## Setup

Using SSH:

```bash
WORK=/private/tmp/nape-rover-example
rm -rf "$WORK"
mkdir -p "$WORK"

git clone git@github.com:attestify/rover-medical-system.git "$WORK/rover-medical-system"
git clone git@github.com:attestify/rover-medical-catalog.git "$WORK/rover-medical-catalog"
git clone git@github.com:attestify/rover-medical-evidence-collection.git "$WORK/rover-medical-evidence-collection"

mkdir -p "$WORK/home"
export HOME="$WORK/home"
cd "$WORK/rover-medical-system"
```

Using HTTPS:

```bash
WORK=/private/tmp/nape-rover-example
rm -rf "$WORK"
mkdir -p "$WORK"

git clone https://github.com/attestify/rover-medical-system.git "$WORK/rover-medical-system"
git clone https://github.com/attestify/rover-medical-catalog.git "$WORK/rover-medical-catalog"
git clone https://github.com/attestify/rover-medical-evidence-collection.git "$WORK/rover-medical-evidence-collection"

mkdir -p "$WORK/home"
export HOME="$WORK/home"
cd "$WORK/rover-medical-system"
```

This guide was validated against the source revisions listed in [example outputs](../examples/README.md).

Check tooling:

```bash
nape --help
nape-eval --check-install
```

## Release 1: Thin Slice

Release 1 verifies one action for the Pet Medicine app.

Start the collection:

```bash
nape collect start \
  --subject "nrn:procedure:rover-medical/rover-medicine-system" \
  --subject-id "localrelease1" \
  --procedure-link "file://$WORK/rover-medical-catalog" \
  --procedure-directory "rover-mediical-system/release-1" \
  --meta system-owner "Bill Bensing"
```

Collect evidence under the `pet-medicine-app` activity:

```bash
nape collect evidence \
  --control-activity "pet-medicine-app" \
  --file-path "pet-medicine-app/app-config.toml"
```

Generate report:

```bash
nape collect report
```

Inspect:

```bash
find . -name assurance_report.yaml -print
sed -n '1,120p' nrn_procedure_rover-medical_-_rover-medicine-system/*/assurance_report.yaml
```

Expected current summary:

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

The current Release 1 test returns `inconclusive` because the database connection test of detail is not implemented in that release.

## Release 3: Expanded Assurance Run

Release 3 adds database, host, cloud, and third-party assurance activities.

Use a fresh workspace or remove the previous generated `nrn_*` output directory before rerunning.

Copy the canned evidence into the system repo:

```bash
cp -R "$WORK/rover-medical-evidence-collection/rover-medical-system/release-3/pet-medicine-db" "$WORK/rover-medical-system/"
cp -R "$WORK/rover-medical-evidence-collection/rover-medical-system/release-3/pet-medicine-host" "$WORK/rover-medical-system/"
cp -R "$WORK/rover-medical-evidence-collection/rover-medical-system/release-3/rover-cloud" "$WORK/rover-medical-system/"
cp -R "$WORK/rover-medical-evidence-collection/rover-medical-system/release-3/exa-doo-dc" "$WORK/rover-medical-system/"
```

Start the collection:

```bash
nape collect start \
  --subject "nrn:procedure:rover-medical/rover-medicine-system" \
  --subject-id "localrelease3" \
  --procedure-link "file://$WORK/rover-medical-catalog" \
  --procedure-directory "rover-mediical-system/release-3" \
  --meta system-owner "Bill Bensing"
```

Collect the app and database evidence under their matching activity names:

```bash
nape collect evidence \
  --control-activity "pet-medicine-app" \
  --file-path "pet-medicine-app/app-config.toml"

nape collect evidence \
  --control-activity "pet-medicine-db" \
  --file-path "pet-medicine-db/my-db.ini"
```

Collect host evidence under the `pet-medicine-host` activity:

```bash
nape collect evidence --control-activity "pet-medicine-host" --file-path "pet-medicine-host/stdout-user-exists-app-user.txt"
nape collect evidence --control-activity "pet-medicine-host" --file-path "pet-medicine-host/stdout-user-exists-db-user.txt"
nape collect evidence --control-activity "pet-medicine-host" --file-path "pet-medicine-host/stdout-no-login-db-user.txt"
nape collect evidence --control-activity "pet-medicine-host" --file-path "pet-medicine-host/stdout-no-login-app-user.txt"
nape collect evidence --control-activity "pet-medicine-host" --file-path "pet-medicine-host/stdout-root-check-admin-user.txt"
nape collect evidence --control-activity "pet-medicine-host" --file-path "pet-medicine-host/stdout-root-check-app-user.txt"
nape collect evidence --control-activity "pet-medicine-host" --file-path "pet-medicine-host/stdout-root-check-db-user.txt"
nape collect evidence --control-activity "pet-medicine-host" --file-path "pet-medicine-host/stdout-auditctl-installed.txt"
nape collect evidence --control-activity "pet-medicine-host" --file-path "pet-medicine-host/stdout-auditctl-user-commands-logged.txt"
```

Collect cloud and third-party evidence under their matching activity names:

```bash
nape collect evidence \
  --control-activity "rover-cloud" \
  --file-path "rover-cloud/cloud-config.yaml"

nape collect evidence \
  --control-activity "exa-doo-dc" \
  --file-path "exa-doo-dc/soc1-report-analysis.json"
```

Generate report:

```bash
nape collect report
```

Expected current summary:

```yaml
summary:
  activity_count: 5
  action_count: 22
  actions_run: 22
  pass: 18
  fail: 0
  inconclusive: 4
  outcome: inconclusive
```

The four inconclusive actions are currently the Rover Cloud and Exa-Doo SOC1 tests that return unimplemented/inconclusive results.

## Notes About The Rover Workflow

The GitHub Actions workflow in `rover-medical-system` performs these high-level steps:

1. Checks out `rover-medical-system`.
2. Checks out `rover-medical-evidence-collection`.
3. Resolves the release from the `RELEASE` file.
4. Installs `nape` and `nape-eval`.
5. Runs Ansible to collect simulated evidence.
6. Runs `nape collect start`.
7. Runs one or more `nape collect evidence` commands.
8. Runs `nape collect report`.
9. Uploads the generated `nrn*` run directory as a workflow artifact.

The local approach in this guide replaces steps 2 and 5 with direct file copies from the canned evidence repository.

## Current Demo Caveats

- The procedure catalog path is currently `rover-mediical-system/...`; use that exact spelling.
- Release 4 is described as a drift demo, but the inspected Release 3 and Release 4 evidence files appeared identical.
- The current SOC1 and Rover Cloud tests return `inconclusive`, so Release 4 drift is not currently proven by those tests.
- Current CLI help may mention uploading results to a repository. The current behavior used by this guide writes `assurance_report.yaml` locally.
