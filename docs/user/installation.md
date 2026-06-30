# Installation

This guide explains how to get both required CLIs available locally:

- `nape`: the Rust CLI from this repository
- `nape-eval`: the evaluator CLI invoked by `nape collect report`

## Official Install Path

The official public install path for the NAPE CLI is the NAPE binary repository.

Download URL shape:

```text
https://repo.napecentral.com/nape-cli/<version>/<operating-system>/<cpu-architecture>/nape
```

Example for a published macOS Apple Silicon binary:

```bash
NAPE_VERSION=1.1.0
curl -L -o nape "https://repo.napecentral.com/nape-cli/${NAPE_VERSION}/macos/aarch64/nape"
chmod +x nape
sudo mv nape /usr/local/bin/nape
nape --version
```

Use the published version, operating system, and CPU architecture for your environment.

Current operating-system values used by the public docs are:

- `linux-gnu`
- `linux-musl`
- `macos`
- `windows`

Current CPU architecture values used by the public docs are:

- `x86_64`
- `aarch64`

Writerside under `../docs/Writerside` is the public documentation source for polished installation flows. This repository keeps current-behavior and developer-local installation documentation.

Integrity note: public binary releases should include checksum, signature, or provenance verification artifacts. Until that release convention is published, follow your organization's software intake controls before installing downloaded binaries onto `PATH`.

## Install NAPE Evaluator

`nape collect report` shells out to `nape-eval`.

Install it with:

```bash
python3 -m pip install nape
```

Verify:

```bash
nape-eval --check-install
```

Expected output:

```text
NAPE Evaluator CLI is installed and working.
```

If your Python environment installs scripts into a user-local bin directory, make sure that directory is on `PATH`.

Common locations:

```text
$HOME/.local/bin
$HOME/Library/Python/<version>/bin
```

## Developer Fallback: Build NAPE From Source

From the repository root:

```bash
cargo build --release -p nape_cli
```

The binary is written to:

```text
target/release/nape
```

Verify directly:

```bash
./target/release/nape --help
```

## Put NAPE On PATH

For temporary use in the current shell:

```bash
export PATH="$PWD/target/release:$PATH"
nape --help
```

For a local user install, copy the binary into a directory already on your `PATH`.

Example:

```bash
mkdir -p "$HOME/.local/bin"
cp target/release/nape "$HOME/.local/bin/nape"
export PATH="$HOME/.local/bin:$PATH"
nape --help
```

If you use a shell profile, add the `PATH` export there.

## Verify Both Tools

```bash
nape --help
nape collect --help
nape collect start --help
nape-eval --check-install
```

## Build And Test The Repository

Build:

```bash
cargo build
```

Run all tests:

```bash
cargo test
```

Run package-specific tests:

```bash
make test-nape-cli
make test-domain
```

## Release Binary Install Target

The Makefile includes:

```bash
make build-release
make install
```

Current behavior:

- `make build-release` builds under the sibling `../builds` directory.
- `make install` copies `../builds/release/nape` to `/usr/local/bin`.

Depending on your machine permissions, copying to `/usr/local/bin` may require elevated privileges. For development, using `target/release` or `$HOME/.local/bin` is usually simpler.

## Version Notes

The CLI currently reports version `1.1.0`.

Check:

```bash
nape --version
```

## Troubleshooting

**`nape: command not found`**

The binary is not on `PATH`. Use:

```bash
./target/release/nape --help
```

or add the binary directory to `PATH`.

**`nape-eval: command not found`**

Install the evaluator:

```bash
python3 -m pip install nape
```

Then confirm the Python scripts directory is on `PATH`.

**`nape collect report` fails install verification**

Run:

```bash
nape-eval --check-install
```

Fix the evaluator install before rerunning `nape collect report`.

**Built binary does not match docs**

Confirm which binary is being executed:

```bash
which nape
nape --version
```

If needed, run the built binary directly:

```bash
./target/release/nape --help
```
