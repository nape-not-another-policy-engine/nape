# Evaluation To Report Traceability

This document traces how V1 procedure inputs become evaluator calls and report fields.

Use this as the report-design bridge between current V1 behavior and future V2 verification report design.

## High-Level Flow

```text
assurance_procedure.yaml
  -> EvaluationFiles
  -> nape-eval command calls
  -> EvaluationResults
  -> AssuranceReportBuilder
  -> SHA256 signed files
  -> assurance_report.yaml
```

## Step Trace

| Step | Input | Code area | Output | Report impact |
| --- | --- | --- | --- | --- |
| Load procedure | `<run-home>/assurance_procedure.yaml` | `retrieve_assurance_procedure` adapter | V1 assurance procedure value | Defines report activities and action names. |
| Build evaluation files | Procedure action `test` and `evidence` fields plus run home | `EvaluationFiles::from` | Map of evidence file path to one or more test file paths | Determines which evaluator calls run. |
| Verify evaluator install | `nape-eval --check-install` | `nape_evidence_evaluator` | Success or fatal error | A failure prevents report generation. |
| Invoke evaluator | `nape-eval --evidence <evidence-file> --test <test-file>` | `nape_evidence_evaluator` | JSON stdout | Produces action outcome and reason. |
| Parse evaluator output | JSON with `outcome` and `reason` | Evaluator adapter | `TestResult` | Invalid JSON or invalid values prevent report generation. |
| Match result to action | Canonical evidence/test path pair | `try_get_test_result` | Action test result | Missing result prevents report generation. |
| Sign evidence | Procedure evidence path plus run home | `try_create_signed_file` | `SignedFile` | Populates `action[].evidence_file`. |
| Sign test | Procedure test path plus run home | `try_create_signed_file` | `SignedFile` | Populates `action[].test_file`. |
| Build report action | Procedure action name plus result and signatures | `AssuranceReportBuilder` | Report action | Populates action name, outcome, reason, signed files. |
| Build summary | Report activities/actions | Kernel report builder | Summary counts and outcome | Populates `summary`. |
| Serialize report | V1 report value | Report serializer | YAML | Writes `assurance_report.yaml`. |

## Procedure Field To Report Field

| Procedure source | Runtime use | Report field |
| --- | --- | --- |
| `procedure.nrn` | Identifies the procedure itself, not directly copied into current report procedure block. | Not emitted as `procedure.nrn` in current report. |
| Start request `--subject` | Subject under evaluation. | `subject.urn` |
| Start request `--subject-id` | Subject instance under evaluation. | `subject.id` |
| Start request `--procedure-link` | Procedure repository source. | `procedure.repository` |
| Start request `--procedure-directory` | Procedure directory source. | `procedure.directory` |
| Start request metadata | User and generated run metadata. | `metadata` |
| `activity[].name` | Groups report actions. | `activity[].name` |
| `action[].name` | Names evaluated action. | `action[].name` |
| `action[].test` | Test file passed to `nape-eval` and signed. | `action[].test_file.file` |
| `action[].evidence` | Evidence file passed to `nape-eval` and signed. | `action[].evidence_file.file` |

## Evaluator JSON To Report Field

Expected evaluator JSON:

```json
{
  "outcome": "pass",
  "reason": "Reason text"
}
```

| Evaluator field | Runtime conversion | Report field |
| --- | --- | --- |
| `outcome` | Converted into kernel `Outcome`. | `action[].outcome` |
| `reason` | Converted into kernel `Description`. | `action[].reason` |

Valid action outcome values:

- `pass`
- `fail`
- `inconclusive`
- `error`

## Failure Modes

| Failure mode | Current V1 result | Report written? | V2 design question |
| --- | --- | --- | --- |
| `nape-eval` missing | Fatal install verification error. | No | Should this be a fatal run error or a report diagnostic? |
| `nape-eval --check-install` non-zero | Fatal install verification error. | No | Should report include environment checks? |
| Evaluator command non-zero | Fatal processing error. | No | Should an evaluator command failure become action outcome `error`? |
| Evaluator stdout is not UTF-8 | Fatal processing error. | No | Should raw stderr/stdout be captured? |
| Evaluator stdout is malformed JSON | Fatal processing error. | No | Should malformed output become action outcome `error`? |
| Evaluator JSON has invalid outcome | Fatal processing error. | No | Should unknown outcome be preserved as diagnostic? |
| Evaluator JSON has invalid reason | Fatal processing error. | No | Should reason constraints be relaxed? |
| Procedure references missing evidence/test file | Fatal during evaluation or signing. | No | Should missing files be action-level outcomes? |
| Action result cannot be matched back to action | Fatal report build error. | No | Should V2 store action IDs in evaluator calls? |

## Shared Evidence Behavior

The current evaluation map groups one evidence path to one or more test paths. This supports multiple actions using the same evidence file.

Example:

```yaml
action:
  - name: database-user
    test: "activity/pet-medicine-app/database-user.py"
    evidence: "evidence/pet-medicine-app/app-config.toml"
  - name: database-user-secret
    test: "activity/pet-medicine-app/database-user-secret.py"
    evidence: "evidence/pet-medicine-app/app-config.toml"
```

Runtime effect:

- The same evidence file is evaluated with multiple test files.
- Each report action still receives its own outcome, reason, test signature, and evidence signature.

## Summary Derivation

Current summary values are derived from built report actions:

- `activity_count`: number of report activities
- `action_count`: number of report actions
- `actions_run`: number of actions with `pass`, `fail`, or `inconclusive` outcomes
- `pass`: number of passing actions
- `fail`: number of failing actions
- `inconclusive`: number of inconclusive actions
- `outcome`: aggregate report outcome

Current aggregate outcome behavior:

- Any `inconclusive` or `error` action makes report outcome `inconclusive`.
- Otherwise, any `fail` action makes report outcome `fail`.
- Otherwise, report outcome is `pass`.

V2 should decide whether:

- `error` gets its own count.
- Fatal evaluator failures are represented in a partial report.
- Report generation can succeed with missing evidence or malformed evaluator output.
- Summary includes skipped, not-run, or blocked action states.
