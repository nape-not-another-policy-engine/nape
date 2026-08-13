# V1 Source Traceability

This reference maps documented V1 behavior to the source files that implement it. Use it when reviewing current behavior, planning V2 verification procedure/report changes, or deciding whether a documentation claim is backed by code.

This document is not a replacement for the user guides or the contract matrix. It is the code-facing source map behind those documents.

## Source Roots

Repository-local sources:

| Source root | Role |
| --- | --- |
| `apps/nape-cli` | CLI entrypoint, command parsing, command handlers, adapters, serializers, runtime state, report persistence. |
| `domain` | Evidence collection use cases, request boundaries, evaluation file mapping, report-building orchestration. |
| `Cargo.lock` | Locked external dependency revisions, including `kernel_oss`. |
| `docs/examples` | Checked-in Rover outputs used to validate documented examples. |
| `scripts/docs-rover-smoke.sh` | Local maintainer smoke validation for documented Rover examples. |

External kernel source:

| Dependency | Locked version | Locked commit | Role |
| --- | --- | --- | --- |
| `kernel_oss` | tag `0.2.5` | `5410baccae23356f1bb59902e7f7afc452577db2` | Core value objects, procedure/report structs, validation rules, summary aggregation, signatures. |

Kernel paths in this document are module paths inside the locked `kernel_oss` source, not vendored files in this repository.

## Command Dispatch

| Behavior | Source |
| --- | --- |
| CLI binary name and top-level version/about text | `apps/nape-cli/src/io_adapter/clap/cli.rs` |
| `collect`, `collect start`, `collect evidence`, `collect report` command tree | `apps/nape-cli/src/io_adapter/clap/cli_commands.rs` |
| CLI argument names, flags, requiredness, and help text | `apps/nape-cli/src/io_adapter/clap/cli_arguments.rs` |
| Command handler dispatch by subcommand name | `apps/nape-cli/src/io_adapter/clap/command_handlers/collect/collect_command_handler.rs` |
| Shared command handler trait | `apps/nape-cli/src/io_adapter/clap/command_handler_boundary.rs` |

Current documentation caveat:

- The top-level and `collect` help text says results are uploaded, but the implemented V1 behavior writes `assurance_report.yaml` locally.
- The `collect evidence` help text says "Control Action", but the flag and behavior use `--control-activity`.

## `collect start` Trace

`nape collect start` creates a run workspace, retrieves the procedure, moves the procedure YAML and activity tests into the workspace, and writes active run state.

| Step | Source | Notes |
| --- | --- | --- |
| Extract CLI arguments | `apps/nape-cli/src/io_adapter/clap/command_handlers/collect/collect_start.rs` | Reads `subject`, `subject-id`, `procedure-link`, `procedure-directory`, and `metadata`. |
| Build start request | `domain/src/evidence_collection/usecases/start_collection/usecase_boundary.rs` | `StartProcedureBuilder` sets `apiVersion` to `1.0.0`, kind to `AssuranceProcedure`, validates subject/procedure, and adds `utc-start`. |
| Validate subject NRN and ID | `kernel_oss/src/values/specification/subject.rs` | Wraps `NRN::new` and `SubjectId::new`. |
| Validate `--subject` NRN | `kernel_oss/src/values/nrn/nrn.rs` | Requires scheme `nrn`, allowed NIDs `procedure` and `sourcecode`, and at least one ASCII/non-whitespace NSS segment. |
| Validate `--subject-id` | `kernel_oss/src/values/specification/subject_id.rs` | Non-empty, alphanumeric only, max 256 characters. |
| Validate procedure repository and directory | `kernel_oss/src/values/specification/procedure.rs` | Allows repository schemes `file`, `git`, `https`; validates procedure directory characters and slash/dash boundaries. |
| Validate repository link | `kernel_oss/src/values/specification/repository_link.rs` | Applies default scheme `git` when no scheme is supplied and rejects unallowed schemes such as `ssh`. |
| Generate workspace directories | `apps/nape-cli/src/usecase_configuration/start_collection.rs` | Creates `home`, `evidence`, `activity-test`, and `temp` directory entries. |
| Retrieve procedure files | `domain/src/evidence_collection/usecases/start_collection/usecase.rs` | Uses allowed schemes `file`, `git`, and `https`; expects `assurance_procedure.yaml` and `activity/`. |
| Select retrieval adapter | `apps/nape-cli/src/usecase_configuration/procedure_retrieval_gateway_factory.rs` | Routes `file` to filesystem copy and `git`/`https` to Git retrieval. |
| Retrieve from local filesystem | `apps/nape-cli/src/gateway_adapter/std_fs/procedure_retrieval_gateway.rs` | Copies a `file://` procedure directory into the temp download directory. |
| Retrieve from Git | `apps/nape-cli/src/gateway_adapter/git2/process_retrieval_gateway.rs` | Performs a shallow bare clone, reads the requested tree, writes the procedure subtree, removes the clone directory. |
| Persist active run state | `apps/nape-cli/src/state_management/cli_app_state.rs`, `apps/nape-cli/src/state_management/yaml_serializer.rs`, `apps/nape-cli/src/usecase_configuration/start_collection.rs` | Writes a YAML state file after the procedure starts. |
| State file path | `apps/nape-cli/src/filesystem_state_configuration.rs` | `$HOME/nape/.nape_cli_config`. |

Current documentation caveat:

- `--meta` is not marked required in the clap argument definition, but `collect_start.rs` unwraps metadata occurrences. Current reliable usage should provide at least one `--meta key value` pair.

## `collect evidence` Trace

`nape collect evidence` copies one user-provided file into the active run evidence directory under the selected activity name.

| Step | Source | Notes |
| --- | --- | --- |
| Extract CLI arguments | `apps/nape-cli/src/io_adapter/clap/command_handlers/collect/collect_evidence.rs` | Reads `--control-activity`, `--file-path`, and optional `--file-name`. |
| Configure use case dependencies | `apps/nape-cli/src/usecase_configuration/collect_evidence.rs` | Wires state lookup, source file read, and filesystem copy gateways. |
| Validate activity name | `domain/src/evidence_collection/usecases/collect_evidence/usecase.rs` | Uses kernel `Name::try_from`. |
| `Name` rules | `kernel_oss/src/values/specification/name.rs` | Non-empty after trim, alphanumeric or dash only, no leading/trailing dash, stored lowercase. |
| Read source file | `apps/nape-cli/src/gateway_adapter/std_fs/retrieve_file_data_gateway.rs` | Requires source file to exist and have a Unicode file name. |
| Resolve target evidence directory | `apps/nape-cli/src/gateway_adapter/state_management/retrieve_directory_path.rs` | Loads the active state file and reads the `evidence` directory entry. |
| Copy file | `apps/nape-cli/src/gateway_adapter/std_fs/copy_file_gateway.rs` | Creates the activity directory and writes file bytes. |

Current documentation caveats:

- `--file-name` overrides the destination file name, but the current command path does not use a kernel file-name value object for validation.
- Evidence is copied under the active run's `evidence/<activity-name>/` directory. The activity name is normalized through the kernel `Name` rules.

## `collect report` Trace

`nape collect report` reads the active state, loads the procedure YAML, maps procedure actions to evidence/test file pairs, invokes `nape-eval`, signs evidence and test files, builds a V1 report, and writes `assurance_report.yaml`.

| Step | Source | Notes |
| --- | --- | --- |
| Build report request from active state | `apps/nape-cli/src/io_adapter/clap/command_handlers/collect/collect_report.rs` | Reads subject, procedure, and metadata from `$HOME/nape/.nape_cli_config`. |
| Configure report dependencies | `apps/nape-cli/src/usecase_configuration/evidence_report.rs` | Wires state lookup, procedure YAML reader, evaluator gateway, SHA256 signer, file reader, and report persistence. |
| Validate report request | `domain/src/evidence_collection/usecases/evaluate_evidence/usecase_boundary.rs` | Revalidates subject, procedure, and metadata before report generation. |
| Retrieve procedure YAML path and home dir | `domain/src/evidence_collection/usecases/evaluate_evidence/usecase.rs` | Reads `assurance-procedure-file` and `home` from active state. |
| Read and deserialize procedure YAML | `apps/nape-cli/src/gateway_adapter/std_fs/retrieve_assurance_procedure.rs` | Opens YAML, deserializes with serde, converts to kernel `AssuranceProcedure`. |
| Map YAML fields to kernel procedure | `apps/nape-cli/src/gateway_adapter/serde/specification_serializer/assurance_procedure/v1_0_0.rs` | Defines current V1 procedure YAML shape. |
| Map action paths to evaluator inputs | `domain/src/evidence_collection/usecases/evaluate_evidence/gateway_boundary.rs` | `EvaluationFiles::from` combines run home with each action's `evidence` and `test` path. |
| Invoke evaluator | `apps/nape-cli/src/gateway_adapter/nape_evaluator/evaluate_evidence_gateway.rs` | Runs `nape-eval --check-install`, then `nape-eval --evidence <file> --test <file>` for each pair. |
| Convert evaluator JSON to action result | `apps/nape-cli/src/gateway_adapter/nape_evaluator/evaluate_evidence_gateway.rs`, `domain/src/evidence_collection/usecases/evaluate_evidence/gateway_boundary.rs` | Expects JSON with `outcome` and `reason`; outcome is parsed by kernel `Outcome`. |
| Build report activities/actions | `domain/src/evidence_collection/usecases/evaluate_evidence/usecase.rs` | Copies procedure activity/action names, evaluator outcome/reason, and signed file references into report actions. |
| Sign files | `domain/src/evidence_collection/usecases/evaluate_evidence/usecase.rs`, `apps/nape-cli/src/gateway_adapter/sha2/signature_algorithm.rs`, `kernel_oss/src/algorithms/signature_algorithm.rs` | Reads bytes from run-home-combined paths, hashes with SHA256, serializes as `SHA256[<hex>]`. |
| Build V1 report object | `kernel_oss/src/values/specification/v1_0_0/assurance_report.rs` | Sets report API version `1.0.0` and computes summary from activities. |
| Serialize report YAML | `apps/nape-cli/src/gateway_adapter/serde/specification_serializer/assurance_report/v1_0_0.rs` | Defines current V1 report YAML field names and omissions. |
| Persist report | `apps/nape-cli/src/gateway_adapter/serde/persist_report_gateway.rs` | Writes `<run-home>/assurance_report.yaml`. |

Current failure behavior:

- If `nape-eval --check-install` fails, report generation fails before action evaluation.
- If a `nape-eval` action invocation exits non-zero, report generation fails instead of writing an action result.
- If evaluator stdout is not valid JSON with the expected fields, report generation fails.
- If evaluator stdout returns a valid `error` outcome, that is an action-level outcome and the report can be built.

## Procedure YAML Source Map

| YAML field | Source |
| --- | --- |
| `apiVersion` | `apps/nape-cli/src/gateway_adapter/serde/specification_serializer/assurance_procedure/v1_0_0.rs`, `kernel_oss/src/values/specification/api_version.rs` |
| `kind` | `apps/nape-cli/src/gateway_adapter/serde/specification_serializer/assurance_procedure/v1_0_0.rs`, `kernel_oss/src/values/specification/kind.rs` |
| `procedure.nrn` | `apps/nape-cli/src/gateway_adapter/serde/specification_serializer/assurance_procedure/v1_0_0.rs`, `kernel_oss/src/values/specification/assurance_procedure/procedure.rs`, `kernel_oss/src/values/nrn/nrn.rs` |
| `procedure.short` | `apps/nape-cli/src/gateway_adapter/serde/specification_serializer/assurance_procedure/v1_0_0.rs`, `kernel_oss/src/values/specification/short_description.rs` |
| `procedure.description` | `apps/nape-cli/src/gateway_adapter/serde/specification_serializer/assurance_procedure/v1_0_0.rs`, `kernel_oss/src/values/specification/description.rs` |
| `activity[].name` | `apps/nape-cli/src/gateway_adapter/serde/specification_serializer/assurance_procedure/v1_0_0.rs`, `kernel_oss/src/values/specification/assurance_procedure/activity.rs`, `kernel_oss/src/values/specification/name.rs` |
| `activity[].short` | `apps/nape-cli/src/gateway_adapter/serde/specification_serializer/assurance_procedure/v1_0_0.rs`, `kernel_oss/src/values/specification/short_description.rs` |
| `activity[].description` | `apps/nape-cli/src/gateway_adapter/serde/specification_serializer/assurance_procedure/v1_0_0.rs`, `kernel_oss/src/values/specification/description.rs` |
| `activity[].action[]` | `apps/nape-cli/src/gateway_adapter/serde/specification_serializer/assurance_procedure/v1_0_0.rs`, `kernel_oss/src/values/specification/assurance_procedure/action.rs` |
| `action[].name` | `apps/nape-cli/src/gateway_adapter/serde/specification_serializer/assurance_procedure/v1_0_0.rs`, `kernel_oss/src/values/specification/name.rs` |
| `action[].short` | `apps/nape-cli/src/gateway_adapter/serde/specification_serializer/assurance_procedure/v1_0_0.rs`, `kernel_oss/src/values/specification/short_description.rs` |
| `action[].description` | `apps/nape-cli/src/gateway_adapter/serde/specification_serializer/assurance_procedure/v1_0_0.rs`, `kernel_oss/src/values/specification/description.rs` |
| `action[].test` | `apps/nape-cli/src/gateway_adapter/serde/specification_serializer/assurance_procedure/v1_0_0.rs`, `kernel_oss/src/values/specification/file_path.rs` |
| `action[].evidence` | `apps/nape-cli/src/gateway_adapter/serde/specification_serializer/assurance_procedure/v1_0_0.rs`, `kernel_oss/src/values/specification/file_path.rs` |

Compatibility note:

- `kernel_oss` has `Activity::add_expected_evidence`, but the current repository-local V1 procedure YAML serializer does not model or populate `expect-evidence`. Treat action-level `evidence` fields as the current V1 source of truth.

## Report YAML Source Map

| Report field | Source |
| --- | --- |
| `apiVersion` | `kernel_oss/src/values/specification/v1_0_0/assurance_report.rs`, `apps/nape-cli/src/gateway_adapter/serde/specification_serializer/assurance_report/v1_0_0.rs` |
| `kind` | `kernel_oss/src/values/specification/traits/mod.rs`, `apps/nape-cli/src/gateway_adapter/serde/specification_serializer/assurance_report/v1_0_0.rs` |
| `metadata` | `domain/src/evidence_collection/usecases/start_collection/usecase_boundary.rs`, `apps/nape-cli/src/state_management/cli_app_state.rs`, `kernel_oss/src/values/specification/metadata.rs` |
| `subject.urn` | `collect start` request/state via `kernel_oss/src/values/specification/subject.rs` |
| `subject.id` | `collect start` request/state via `kernel_oss/src/values/specification/subject_id.rs` |
| `procedure.repository` | `collect start` request/state via `kernel_oss/src/values/specification/procedure.rs` |
| `procedure.directory` | `collect start` request/state via `kernel_oss/src/values/specification/procedure.rs` |
| `summary.*` | `kernel_oss/src/values/specification/assurance_report/summary.rs` |
| `activity[].name` | Procedure activity name copied in `domain/src/evidence_collection/usecases/evaluate_evidence/usecase.rs` |
| `action[].name` | Procedure action name copied in `domain/src/evidence_collection/usecases/evaluate_evidence/usecase.rs` |
| `action[].outcome` | Evaluator JSON parsed in `apps/nape-cli/src/gateway_adapter/nape_evaluator/evaluate_evidence_gateway.rs`, validated by `kernel_oss/src/values/specification/outcome.rs` |
| `action[].reason` | Evaluator JSON parsed in `apps/nape-cli/src/gateway_adapter/nape_evaluator/evaluate_evidence_gateway.rs`, validated by `kernel_oss/src/values/specification/description.rs` |
| `action[].test_file.file` | Procedure `action[].test`, preserved as non-canonical path in `domain/src/evidence_collection/usecases/evaluate_evidence/usecase.rs` |
| `action[].test_file.signature` | SHA256 over the run-home-combined test file bytes. |
| `action[].evidence_file.file` | Procedure `action[].evidence`, preserved as non-canonical path in `domain/src/evidence_collection/usecases/evaluate_evidence/usecase.rs` |
| `action[].evidence_file.signature` | SHA256 over the run-home-combined evidence file bytes. |
| `additional_information` | `kernel_oss/src/values/specification/assurance_report/additional_information.rs`, serialized only when non-empty. |

## Summary Logic Source Map

Summary aggregation is implemented in `kernel_oss/src/values/specification/assurance_report/summary.rs`.

| Summary field | Current behavior |
| --- | --- |
| `activity_count` | Number of report activities. |
| `action_count` | `pass + fail + inconclusive + error`. |
| `actions_run` | `pass + fail + inconclusive`; `error` is excluded. |
| `pass` | Count of action outcomes equal to `pass`. |
| `fail` | Count of action outcomes equal to `fail`. |
| `inconclusive` | Count of action outcomes equal to `inconclusive`. |
| `outcome` | `inconclusive` if any action is `inconclusive` or `error`; otherwise `fail` if any action is `fail`; otherwise `pass`. |

The report serializer does not expose a separate `error` count in V1.

## Validation Rule Source Map

| Contract | Source | Current rule |
| --- | --- | --- |
| Subject NRN | `kernel_oss/src/values/nrn/nrn.rs` | Format `nrn:<nid>:<nss>`; allowed NIDs `procedure`, `sourcecode`; NSS segments must be ASCII and contain no whitespace. |
| Subject ID | `kernel_oss/src/values/specification/subject_id.rs` | Required, max 256 chars, alphanumeric only. |
| Activity/action names | `kernel_oss/src/values/specification/name.rs` | Required, alphanumeric or dash only, no leading/trailing dash, lowercased. |
| Procedure repository link | `kernel_oss/src/values/specification/repository_link.rs` and `kernel_oss/src/values/specification/procedure.rs` | Default scheme `git`; allowed schemes `file`, `git`, `https`. |
| Procedure directory | `kernel_oss/src/values/specification/procedure.rs` | Empty or `/` accepted as `/`; otherwise alphanumeric, `_`, `-`, `/`; no leading/trailing slash, no contiguous slash, no leading/trailing dash. |
| Procedure action paths | `kernel_oss/src/values/specification/file_path.rs` | Non-empty only. Workspace-relative convention is enforced by docs and runtime composition, not by a strict path grammar. |
| Evaluator outcomes | `kernel_oss/src/values/specification/outcome.rs` | Case-insensitive parse of `pass`, `fail`, `inconclusive`, `error`; serialized lowercase. |
| Signatures | `apps/nape-cli/src/gateway_adapter/sha2/signature_algorithm.rs`, `kernel_oss/src/algorithms/signature_algorithm.rs` | Non-empty SHA256 hex digest serialized as `SHA256[<hex>]`. |

## V2 Planning Pivots

Use these source-backed gaps as the starting point for V2 planning:

| Gap | Current source-backed behavior | V2 decision needed |
| --- | --- | --- |
| CLI upload wording | Help text says upload; implementation persists locally. | Update help, implement upload, or explicitly split local/export/upload commands. |
| Metadata optionality | Clap does not require `--meta`; handler unwraps occurrences. | Decide whether metadata is optional, required, or defaulted. |
| `ssh://` repositories | Repository link allows `file`, `git`, `https`; `ssh` is rejected. | Decide whether SSH is supported and how credentials/known hosts are handled. |
| Procedure path grammar | `FilePath` only rejects empty paths; docs recommend workspace-relative safe paths. | Define relative path grammar, normalization, symlink behavior, absolute path rejection, and `../` handling. |
| Error accounting | `error` action outcomes affect `action_count` and overall outcome but have no separate report count. | Add explicit error count or new outcome taxonomy if needed. |
| Fatal evaluator failures | Install failure, non-zero process exit, and malformed JSON prevent report generation. | Decide which failures are reportable as action results versus fatal command errors. |
| `expect-evidence` | Kernel has expected-evidence support, but current YAML serializer ignores the field. | Remove, formalize, or migrate it in V2 procedures. |
| Git source reproducibility | Report records repository and directory, not resolved commit/ref. | Add resolved source revision/provenance if V2 reports must be reproducible. |
| Single active run | State file stores one active run under `$HOME/nape/.nape_cli_config`. | Keep simple state, add run IDs, or support multiple active runs. |

