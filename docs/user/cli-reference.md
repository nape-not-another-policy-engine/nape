# NAPE CLI 2.0 Reference

```text
Usage: NAPE CLI <COMMAND>

Commands:
  package
  start
  evidence
  verify
  help
```

## `nape package build`

```text
nape package build --source <DIRECTORY> --package <PURL>
  [--dependency-package <BUILD_RESULT_DIRECTORY>]...
  --output <NEW_DIRECTORY>
```

Builds one exact Verification Action, Activity, or Procedure package. Authored
source cannot contain `attestify-lock.json`. NAPE generates the canonical Lock
from the complete exact dependency-package set. Construction performs no
network or neighboring-directory resolution.

## `nape package publish`

```text
nape package publish --local-package <BUILD_RESULT_DIRECTORY>
  (--registry-endpoint <URL> | --registry-profile <REGISTRY_MAP_FILE>)
```

Publishes an admitted build result without rebuilding it, then verifies a
clean digest-selected re-pull.

## `nape package resolve`

```text
nape package resolve --package <PURL> --manifest-digest <SHA256_DIGEST>
  (--registry-endpoint <URL> | --registry-profile <REGISTRY_MAP_FILE>)
  --plan-only
```

Maps and verifies the exact root release, then emits the canonical Lock plan.
It does not admit evidence or execute a Procedure.

## `nape start`

Local source:

```text
nape start --local-package <BUILD_RESULT_DIRECTORY>
  --subject-file <FILE> [--meta <KEY> <VALUE>]...
```

OCI source:

```text
nape start --package <PURL> --manifest-digest <SHA256_DIGEST>
  (--registry-endpoint <URL> | --registry-profile <REGISTRY_MAP_FILE>)
  --subject-file <FILE> [--meta <KEY> <VALUE>]...
```

The sources are mutually exclusive. Start freezes the verified closure,
subject, metadata, effective graph, and NAPE-managed output location in one new
current run. Success is silent.

## `nape evidence`

```text
nape evidence --action <ACTIVITY.ACTION> --file <SOURCE_FILE>
  [--file-name <DEFINITION_FILE_NAME>]
```

Adds or replaces one payload in the current collecting run. The source
basename may differ from the definition-owned filename. `--file-name` asserts,
but does not rename, that definition filename. Success is silent.

## `nape verify`

```text
nape verify
```

Executes the current run without package re-resolution. Required evidence must
be complete before Action one. Success atomically commits the Report, Evidence
Set relationship, and digest-addressed raw evidence to the NAPE-managed result
directory named in the Receipt.

## Registry input

`--registry-endpoint` currently accepts exact anonymous development endpoints
of the form `http://localhost:<canonical-port>`. It supplies one command-local
mapping for every `pkg:attestify` publisher and the fixed repository prefix
`attestify`.

`--registry-profile` admits the closed `attestify-oci-registry-map/1` file with
explicit publisher rows. Exactly one form is required for OCI operations.

## Receipts and exits

Package and Verify dispatched success use exit `0` and one succeeded Receipt
V2 line. Their dispatched failures use exit `1` and one failed Receipt when
emission remains possible. Start and Evidence are silent on success and emit a
bounded diagnostic on failure. Clap grammar failure uses exit `2` and no
Receipt V2.
