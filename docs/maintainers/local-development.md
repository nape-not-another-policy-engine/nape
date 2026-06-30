# Local Development

This guide covers local development and validation for the NAPE CLI repository.

## Repository Layout

```text
.
  apps/nape-cli/
  domain/
  Cargo.toml
  Makefile
  README.md
  docs/
```

Workspace crates:

- `nape_cli`
- `nape_domain`

## Build

Build all workspace members:

```bash
cargo build
```

Build the release binary:

```bash
cargo build --release -p nape_cli
```

Release binary:

```text
target/release/nape
```

The Makefile also provides:

```bash
make build-release
```

That target writes release artifacts under the sibling `../builds` directory.

## Release Artifact Integrity Follow-Up

The official public CLI install path is the binary repository documented in `docs/user/installation.md`.

Current docs intentionally do not provide checksum, signature, or provenance verification commands because the release artifact convention has not been published yet.

For V2 or the next release-process hardening pass, define and publish one of:

- SHA256 checksum files next to each binary.
- A signed checksum manifest per release.
- Signed provenance or attestations in addition to checksums.

After the release convention exists, update Writerside public installation docs and `docs/user/installation.md` with the actual verification steps.

## Test

Run all tests:

```bash
cargo test
```

Run package-specific tests:

```bash
make test-nape-cli
make test-domain
```

The full Makefile test target enables coverage instrumentation:

```bash
make test
```

## External Dependencies

The CLI uses these Git dependencies:

- `kernel_oss` from `https://github.com/attestify/kernel-oss.git`, tag `0.2.5`
- `test_framework_oss` from `https://github.com/attestify/test-framework-oss.git`, tag `0.2.4`

The current `Cargo.lock` pins `kernel_oss` to:

```text
5410baccae23356f1bb59902e7f7afc452577db2
```

Report generation also requires the external evaluator:

```bash
nape-eval --check-install
```

Install command used by the Rover workflow:

```bash
python3 -m pip install nape
```

## Running A Local CLI Smoke Test

Use a temp workspace and isolated `HOME` so local smoke tests do not overwrite your normal NAPE state:

```bash
WORK=/private/tmp/nape-dev-smoke
rm -rf "$WORK"
mkdir -p "$WORK/home"
export HOME="$WORK/home"
```

Clone examples:

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

Known-good example commits are listed in `docs/examples/README.md`.

Run Release 1:

```bash
cd "$WORK/rover-medical-system"

nape collect start \
  --subject "nrn:procedure:rover-medical/rover-medicine-system" \
  --subject-id "localrelease1" \
  --procedure-link "file://$WORK/rover-medical-catalog" \
  --procedure-directory "rover-mediical-system/release-1" \
  --meta system-owner "Bill Bensing"

nape collect evidence \
  --control-activity "pet-medicine-app" \
  --file-path "pet-medicine-app/app-config.toml"

nape collect report
```

Inspect:

```bash
find . -name assurance_report.yaml -print
sed -n '1,120p' nrn_procedure_rover-medical_-_rover-medicine-system/*/assurance_report.yaml
```

## Running The Expanded Rover Smoke Test

For Release 3, also clone the evidence collection repo:

```bash
git clone git@github.com:attestify/rover-medical-evidence-collection.git "$WORK/rover-medical-evidence-collection"
```

Copy canned evidence into the system repo:

```bash
cp -R "$WORK/rover-medical-evidence-collection/rover-medical-system/release-3/pet-medicine-db" "$WORK/rover-medical-system/"
cp -R "$WORK/rover-medical-evidence-collection/rover-medical-system/release-3/pet-medicine-host" "$WORK/rover-medical-system/"
cp -R "$WORK/rover-medical-evidence-collection/rover-medical-system/release-3/rover-cloud" "$WORK/rover-medical-system/"
cp -R "$WORK/rover-medical-evidence-collection/rover-medical-system/release-3/exa-doo-dc" "$WORK/rover-medical-system/"
```

Then follow the Release 3 commands in:

```text
docs/user/rover-medical-example.md
```

Current expected Release 3 summary:

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

## Documentation Verification

When updating docs, verify:

```bash
rg -n "nape start collect|nape collect start" README.md docs
rg -n "ssh://|--subject-id|--procedure-link|--meta" docs
```

Confirm examples still use:

- `nape collect start`, not `nape start collect`
- Alphanumeric subject IDs
- `file://`, `git://`, or `https://` procedure links
- At least one `--meta` pair
- Current Rover catalog path spelling: `rover-mediical-system`

## Useful Test Targets

```bash
make test-nape-cli
make test-domain
make docs-rover-smoke
cargo test -p nape_cli --lib
cargo test -p nape_domain --lib
```

`make docs-rover-smoke` is a local maintainer validation target. Keep it out of required CI until there is a fixture strategy that avoids external network flakiness or intentionally accepts it.

## Dirty Worktree Awareness

This repository may have unrelated local changes. Before editing, check:

```bash
git status --short
```

Do not revert unrelated user changes when updating docs or code.
