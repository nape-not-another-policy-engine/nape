# V1 Contract Matrix

This reference captures the current V1 contracts that matter for V2 planning. It complements the task-oriented docs and the product baseline.

For implementation-level source mapping behind these contracts, see `v1-source-traceability.md`.

## Procedure File Contract

File name:

```text
assurance_procedure.yaml
```

Procedure repository layout:

```text
<procedure-directory>/
  assurance_procedure.yaml
  activity/
```

| Field | Required | Type | Cardinality | Producer | Consumer | Validation owner | Current behavior | V2 note |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `apiVersion` | Yes | String | One | Procedure author | Procedure serializer | Kernel/API version value | Current examples use `1.0.0`. | Decide accepted V1 versions and V2 versioning. |
| `kind` | Yes | String | One | Procedure author | Procedure serializer | Serializer/kernel kind conversion | Current value is `AssuranceProcedure`. | Keep or introduce explicit V2 kind/version pairing. |
| `procedure.nrn` | Yes | NRN string | One | Procedure author | Procedure serializer | Kernel NRN value | `nrn:<nid>:<nss>`; current allowed NIDs include `procedure` and `sourcecode`. | Decide if V2 expands NID set. |
| `procedure.short` | Yes | String | One | Procedure author | Procedure serializer | Kernel short description value | Human-readable label. | Keep as display metadata. |
| `procedure.description` | Yes | String | One | Procedure author | Procedure serializer | Kernel description value | Longer human-readable description. | Keep as display metadata. |
| `activity` | Yes | List | One list | Procedure author | Procedure serializer | Kernel activities builder | Contains activity entries. | Decide whether empty activity lists are rejected explicitly. |
| `activity[].name` | Yes | Name string | One per activity | Procedure author | Procedure serializer, CLI evidence collection | Kernel name value | Non-empty, alphanumeric/dash only, no leading/trailing dash, stored lowercase. | Keep stable; consider clearer error messages. |
| `activity[].short` | Yes | String | One per activity | Procedure author | Procedure serializer | Kernel short description value | Human-readable label. | Keep. |
| `activity[].description` | Yes | String | One per activity | Procedure author | Procedure serializer | Kernel description value | Longer description. | Keep. |
| `activity[].action` | Yes | List | One list per activity | Procedure author | Procedure serializer | Kernel action builder | Contains action entries. | Decide whether empty action lists are rejected explicitly. |
| `action[].name` | Yes | Name string | One per action | Procedure author | Procedure serializer, report builder | Kernel name value | Non-empty, alphanumeric/dash only, no leading/trailing dash, stored lowercase. | Keep stable. |
| `action[].short` | Yes | String | One per action | Procedure author | Procedure serializer | Kernel short description value | Human-readable label. | Keep. |
| `action[].description` | Yes | String | One per action | Procedure author | Procedure serializer | Kernel description value | Longer description. | Keep. |
| `action[].test` | Yes | File path string | One per action | Procedure author | Evaluation file mapping, evaluator, signer | Kernel file path value and filesystem read | Workspace-relative test path is expected. | Define V2 path grammar. |
| `action[].evidence` | Yes | File path string | One per action | Procedure author | Evaluation file mapping, evaluator, signer | Kernel file path value and filesystem read | Workspace-relative evidence path is expected. Multiple actions may share one evidence path. | Define sharing and missing-evidence behavior. |
| `activity[].expect-evidence` | No | Unknown | Ignored by current serializer model | Older examples | Not modeled as required by current serializer | None in current V1 serializer | Should not be treated as required for current runs. | Decide whether V2 removes, formalizes, or migrates it. |

## Report File Contract

File name:

```text
assurance_report.yaml
```

Output location:

```text
<run-home>/assurance_report.yaml
```

| Field | Required | Type | Cardinality | Producer | Consumer | Validation owner | Current behavior | V2 note |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `apiVersion` | Yes | String | One | Report serializer | Report readers | Kernel report value | Current generated value is `1.0.0`. | Decide V2 report version and V1 compatibility. |
| `kind` | Yes | String | One | Report serializer | Report readers | Kernel report value | Current generated value is `AssuranceReport`. | Keep or version with kind. |
| `metadata` | Conditional | Map string to string | Zero or one map | Start request plus generated state | Report readers | Kernel metadata values | Serializer omits when empty. Current CLI path expects at least one user metadata pair. | Decide optional metadata behavior. |
| `metadata.utc-start` | Yes in current CLI-created runs | String timestamp | One | Start collection builder | State/report readers | Start time value | UTC millisecond start time serialized as string. | Decide timestamp type and naming. |
| `subject.urn` | Yes | NRN string | One | Start request | Report readers | Kernel subject value | Copied from `--subject`. | Keep. |
| `subject.id` | Yes | Subject ID string | One | Start request | Report readers | Kernel subject ID value | Non-empty, alphanumeric, max 256 chars. | Decide if V2 permits richer IDs. |
| `procedure.repository` | Yes | String | One | Start request | Report readers | Kernel procedure value | Copied from `--procedure-link`. | Consider commit/ref capture for reproducibility. |
| `procedure.directory` | Yes | String | One | Start request | Report readers | Kernel procedure value | Copied from `--procedure-directory`. | Keep and consider normalized path. |
| `summary.activity_count` | Yes | Integer | One | Kernel report summary | Report readers | Kernel summary builder | Number of report activities. | Keep. |
| `summary.action_count` | Yes | Integer | One | Kernel report summary | Report readers | Kernel summary builder | Number of actions in report, including `error` outcomes. | Keep. |
| `summary.actions_run` | Yes | Integer | One | Kernel report summary | Report readers | Kernel summary builder | Count of `pass`, `fail`, and `inconclusive`; `error` is not counted. | Revisit for V2. |
| `summary.pass` | Yes | Integer | One | Kernel report summary | Report readers | Kernel summary builder | Count of passing actions. | Keep. |
| `summary.fail` | Yes | Integer | One | Kernel report summary | Report readers | Kernel summary builder | Count of failing actions. | Keep. |
| `summary.inconclusive` | Yes | Integer | One | Kernel report summary | Report readers | Kernel summary builder | Count of inconclusive actions. | Keep. |
| `summary.outcome` | Yes | Outcome string | One | Kernel report summary | Report readers | Kernel outcome value | Overall outcome. Any `inconclusive` or `error` makes report outcome `inconclusive`; otherwise any `fail` makes `fail`; otherwise `pass`. | Consider explicit error accounting. |
| `activity[]` | Yes | List | One list | Report builder | Report readers | Kernel report activities | Generated from procedure activities with evaluated action results. | Keep. |
| `activity[].name` | Yes | Name string | One per activity | Report builder | Report readers | Kernel activity value | Copied from procedure activity. | Keep. |
| `activity[].action[]` | Yes | List | One per activity | Report builder | Report readers | Kernel action value | Generated from procedure actions. | Keep. |
| `action[].name` | Yes | Name string | One per action | Report builder | Report readers | Kernel action value | Copied from procedure action. | Keep. |
| `action[].outcome` | Yes | Outcome string | One per action | `nape-eval` result conversion | Report readers | Kernel outcome value | Lowercase serialized outcome. | Keep accepted values unless V2 adds outcome taxonomy. |
| `action[].reason` | Yes | String | One per action | `nape-eval` result conversion | Report readers | Kernel description value | Human-readable reason from evaluator JSON. | Decide reason length/format contract. |
| `action[].test_file.file` | Yes | File path string | One per action | Report builder | Report readers | Kernel signed file value | Procedure-relative test path. | Keep; consider normalized path. |
| `action[].test_file.signature` | Yes | Signature string | One per action | SHA256 signer | Report readers | Kernel signed file value | `SHA256[<hex>]`. | Decide if V2 supports algorithms other than SHA256. |
| `action[].evidence_file.file` | Yes | File path string | One per action | Report builder | Report readers | Kernel signed file value | Procedure-relative evidence path. | Keep; consider normalized path. |
| `action[].evidence_file.signature` | Yes | Signature string | One per action | SHA256 signer | Report readers | Kernel signed file value | `SHA256[<hex>]`. | Decide if V2 supports algorithms other than SHA256. |
| `additional_information` | Conditional | List of strings | Zero or one list | Kernel report | Report readers | Serializer omits when empty | Present only when kernel report has additional info. | Decide whether V2 formalizes diagnostics. |

## CLI Input Contract

| Input | Required | Current validation | Current behavior | V2 note |
| --- | --- | --- | --- | --- |
| `--subject` | Yes | NRN value | Identifies evaluated subject. | Keep; consider clearer subject/procedure distinction. |
| `--subject-id` | Yes | Non-empty, alphanumeric only, max 256 chars | Identifies evaluated subject instance. | Decide if V2 supports dashes/ULIDs/UUIDs. |
| `--procedure-link` | Yes | Repository link value | Supports `file`, `git`, and `https`; rejects `ssh://`. | Decide SSH support and commit pinning. |
| `--procedure-directory` | Yes | Procedure directory value | Empty or `/` accepted; otherwise no leading/trailing slash, no `//`, allowed chars alnum, `_`, `-`, `/`. | Decide stricter relative path grammar. |
| `--meta` | Help says optional, current handler expects present | Metadata key/value values | One or more pairs should be supplied in current examples. | Decide optionality and default metadata. |
| `--control-activity` | Yes for evidence collection | Name value | Evidence copied under `evidence/<activity-name>/`. | Keep user-facing activity term. |
| `--file-path` | Yes for evidence collection | Filesystem read path | Source evidence file to copy. | Decide path validation and error messages. |
| `--file-name` | No | No kernel value object in current command path | Overrides destination file name. | Decide filename validation. |

## Runtime State Contract

State file:

```text
$HOME/nape/.nape_cli_config
```

| Field | Required | Producer | Consumer | Current behavior | V2 note |
| --- | --- | --- | --- | --- | --- |
| `subject_nrn` | Yes | `collect start` | `collect evidence`, `collect report` | Active run subject. | Keep or replace with explicit run records. |
| `subject_id` | Yes | `collect start` | `collect evidence`, `collect report` | Active run subject ID. | Keep. |
| `procedure_repository` | Yes | `collect start` | `collect report` | Procedure repository link. | Consider recording resolved commit/ref. |
| `procedure_directory` | Yes | `collect start` | `collect report` | Procedure directory. | Keep. |
| `metadata` | Yes in current CLI-created state | `collect start` | `collect report` | User metadata plus `utc-start`. | Decide optional metadata. |
| `directories.home` | Yes | `collect start` | `collect evidence`, `collect report` | Run workspace root. | Keep or replace with run ID index. |
| `directories.evidence` | Yes | `collect start` | `collect evidence` | Evidence root. | Keep. |
| `directories.activity-test` | Yes | `collect start` | `collect report` | Activity test root. | Naming could be clarified in V2. |
| `directories.temp` | Yes | `collect start` | Start cleanup | Temporary retrieval directory. | Decide whether temp state should remain after cleanup. |
| `directories.assurance-procedure-file` | Yes | `collect start` | `collect report` | Retrieved procedure file. | Keep. |

## Evaluator Contract

NAPE invokes:

```bash
nape-eval --check-install
nape-eval --evidence <evidence-file> --test <test-file>
```

Expected evaluator stdout JSON:

```json
{
  "outcome": "pass",
  "reason": "Reason text"
}
```

| Contract point | Current behavior | V2 note |
| --- | --- | --- |
| Install check | Report generation fails before action evaluation if `nape-eval --check-install` fails. | Decide whether install diagnostics are reportable or fatal. |
| Command failure | Non-zero `nape-eval` exit is a processing failure and prevents report generation. | Decide if command errors become action-level `error` outcomes. |
| Malformed JSON | Deserialization failure prevents report generation. | Decide if malformed evaluator output is reportable. |
| Outcome parsing | Outcome values are parsed by kernel outcome value; generated reports serialize lowercase. | Keep or extend outcome taxonomy. |
| Reason parsing | Reason is parsed as a description. | Define V2 reason length/format constraints. |

## Path Contract

Current safe authoring convention:

- Use paths relative to the run workspace.
- Use `activity/<activity-name>/<test-file>` for tests.
- Use `evidence/<activity-name>/<evidence-file>` for evidence.
- Avoid absolute paths.
- Avoid `../`.
- Avoid paths outside the run workspace.

V2 should explicitly decide:

- Whether `./` is normalized or rejected.
- Whether `../` is rejected.
- Whether absolute paths are rejected.
- Whether symlinks are allowed.
- Whether report paths are original procedure paths or normalized paths.
- Whether duplicate separators are rejected or normalized.
