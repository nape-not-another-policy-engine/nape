# Maintainer Architecture

This document maps the current NAPE CLI implementation to its domain use cases and gateway adapters.

## Workspace Shape

The Rust workspace has two members:

```text
apps/nape-cli
domain
```

`apps/nape-cli` owns CLI parsing, command handlers, filesystem adapters, serializers, evaluator integration, and concrete use case wiring.

`domain` owns evidence collection use cases and use case boundaries.

The shared value-object contracts come from `kernel_oss` tag `0.2.5`, locked to commit `5410baccae23356f1bb59902e7f7afc452577db2`.

## Command Entry Point

CLI execution starts in:

```text
apps/nape-cli/src/main.rs
```

Flow:

1. `main` calls `cli::run()`.
2. Clap parses command arguments.
3. `handle_command_results` dispatches the top-level `collect` command.
4. `CollectCommandHandler` dispatches `start`, `evidence`, or `report`.

## CLI Command Modules

Command definitions:

```text
apps/nape-cli/src/io_adapter/clap/cli.rs
apps/nape-cli/src/io_adapter/clap/cli_commands.rs
apps/nape-cli/src/io_adapter/clap/cli_arguments.rs
```

Command handlers:

```text
apps/nape-cli/src/io_adapter/clap/command_handlers/collect/collect_start.rs
apps/nape-cli/src/io_adapter/clap/command_handlers/collect/collect_evidence.rs
apps/nape-cli/src/io_adapter/clap/command_handlers/collect/collect_report.rs
```

Handler responsibilities:

- Extract CLI arguments.
- Build domain request objects.
- Invoke configured use case functions.
- Return `kernel_oss::error::Error` on failure.

## Use Case Wiring

Concrete use case factories live in:

```text
apps/nape-cli/src/usecase_configuration/
```

Current factories:

| Factory | Purpose |
| --- | --- |
| `start_collection::factory_std_fs_git2` | Starts a run using filesystem, Git, and state-file adapters. |
| `collect_evidence::std_fs_factory` | Copies evidence into the active run workspace. |
| `evidence_report::std_fs_factory` | Loads procedure/evidence, invokes evaluator, signs files, and writes report YAML. |

## Domain Use Cases

Start collection:

```text
domain/src/evidence_collection/usecases/start_collection/
```

Collect evidence:

```text
domain/src/evidence_collection/usecases/collect_evidence/
```

Evaluate evidence:

```text
domain/src/evidence_collection/usecases/evaluate_evidence/
```

The domain uses function pointer gateway boundaries. The CLI crate supplies concrete implementations.

## Start Collection Flow

Command:

```bash
nape collect start ...
```

Implementation flow:

1. `StartCollectionCommandHandler` extracts args.
2. It builds `StartProcedure` using `StartProcedureBuilder`.
3. The builder validates values through `kernel_oss`.
4. `factory_std_fs_git2` generates a directory list.
5. `start_collection` creates directories.
6. Procedure files are retrieved from Git or local filesystem.
7. `assurance_procedure.yaml` is moved into the run home.
8. `activity/` is moved into the run home.
9. CLI app state is serialized to `$HOME/nape/.nape_cli_config`.

Key adapters:

```text
apps/nape-cli/src/gateway_adapter/git2/process_retrieval_gateway.rs
apps/nape-cli/src/gateway_adapter/std_fs/procedure_retrieval_gateway.rs
apps/nape-cli/src/gateway_adapter/std_fs/directory_creation_gateway.rs
apps/nape-cli/src/state_management/write_state_file.rs
apps/nape-cli/src/state_management/yaml_serializer.rs
```

## Collect Evidence Flow

Command:

```bash
nape collect evidence ...
```

Implementation flow:

1. `CollectEvidenceCommandHandler` extracts args.
2. `collect_action_evidence` validates the requested activity/control name as a `Name`.
3. The source file is read.
4. Active run state is used to locate the evidence root.
5. The file is copied to `<evidence-root>/<activity-name>/`.

The current domain request field is named `action_name`, but the CLI flag is `--control-activity` and the copy target is an activity directory. Treat this as user-facing activity terminology until the code names are cleaned up.

Key adapters:

```text
apps/nape-cli/src/gateway_adapter/state_management/retrieve_directory_path.rs
apps/nape-cli/src/gateway_adapter/std_fs/retrieve_file_data_gateway.rs
apps/nape-cli/src/gateway_adapter/std_fs/copy_file_gateway.rs
```

## Report Flow

Command:

```bash
nape collect report
```

Implementation flow:

1. `EvaluateAndReportCommandHandler` reads active app state.
2. It builds `EvaluateEvidence`.
3. `evaluate_and_report` loads the retrieved assurance procedure.
4. `EvaluationFiles::from` maps procedure actions to evidence/test paths.
5. `nape_evidence_evaluator` invokes `nape-eval`.
6. `AssuranceReportBuilder` builds report activities and signs files.
7. `save_report_as_yaml` serializes and writes `assurance_report.yaml`.

Key adapters:

```text
apps/nape-cli/src/gateway_adapter/nape_evaluator/evaluate_evidence_gateway.rs
apps/nape-cli/src/gateway_adapter/sha2/signature_algorithm.rs
apps/nape-cli/src/gateway_adapter/std_fs/retrieve_assurance_procedure.rs
apps/nape-cli/src/gateway_adapter/serde/persist_report_gateway.rs
apps/nape-cli/src/gateway_adapter/serde/specification_serializer/
```

## State Management

Active run state is represented by:

```text
apps/nape-cli/src/state_management/cli_app_state.rs
```

State path:

```text
$HOME/nape/.nape_cli_config
```

The state file is YAML and is used by `collect evidence` and `collect report` to resolve the current run.

Implication: the CLI currently tracks one active run per `HOME`.

## Procedure Retrieval

Procedure retrieval supports these schemes:

- `file`
- `git`
- `https`

The factory lives in:

```text
apps/nape-cli/src/usecase_configuration/procedure_retrieval_gateway_factory.rs
```

`file://` uses the local filesystem gateway.

`git://` and `https://` use the `git2` gateway. The implementation shallow-clones the repository, extracts the requested procedure directory, writes the selected tree to disk, then removes the clone directory.

## Evaluator Integration

Report generation shells out to:

```text
nape-eval
```

Before evaluating files, the adapter runs:

```bash
nape-eval --check-install
```

For each action, it runs:

```bash
nape-eval --evidence <evidence-file> --test <test-file>
```

The evaluator output is expected to be JSON with:

```json
{
  "outcome": "pass",
  "reason": "Reason text"
}
```

The outcome is converted into the kernel `Outcome` value.

## Current Risks And Known Issues

- `collect_start.rs` unwraps metadata occurrences even though `--meta` is presented as optional.
- Successful CLI commands usually print no output, making file inspection the main confirmation path.
- CLI help text currently mentions uploading results, but the current implementation writes reports locally.
- `ssh://` procedure links are rejected by the kernel repository link contract.
- The active state file means concurrent runs require isolated `HOME` values or careful sequencing.
- Release 4 drift is described in the Rover README, but the inspected current evidence/tests do not prove a distinct drift outcome.
