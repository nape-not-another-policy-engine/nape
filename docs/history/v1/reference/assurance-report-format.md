# Assurance Report Format

`nape collect report` writes an assurance report to:

```text
<run-home>/assurance_report.yaml
```

The report records the evaluated subject, procedure source, metadata, summary counts, activity/action outcomes, and signatures for the test and evidence files used during evaluation.

## Top-Level YAML Shape

```yaml
apiVersion: 1.0.0
kind: AssuranceReport
metadata:
  utc-start: '1782835488890'
  system-owner: Bill Bensing
subject:
  urn: nrn:procedure:rover-medical/rover-medicine-system
  id: localrelease3
procedure:
  repository: file:///private/tmp/rover-medical-catalog
  directory: rover-mediical-system/release-3
summary:
  activity_count: 5
  action_count: 22
  actions_run: 22
  pass: 18
  fail: 0
  inconclusive: 4
  outcome: inconclusive
activity:
  - name: pet-medicine-app
    action:
      - name: database-connection
        outcome: pass
        reason: The application connects to the database at the expected network host.
        test_file:
          file: activity/pet-medicine-app/database-connection.py
          signature: SHA256[...]
        evidence_file:
          file: evidence/pet-medicine-app/app-config.toml
          signature: SHA256[...]
```

## Fields

| Field | Description |
| --- | --- |
| `apiVersion` | Report API version. Current generated value is `1.0.0`. |
| `kind` | Report kind. Current generated value is `AssuranceReport`. |
| `metadata` | Metadata from the start request plus generated `utc-start`. |
| `subject.urn` | Subject NRN from the start request. |
| `subject.id` | Subject ID from the start request. |
| `procedure.repository` | Procedure repository link from the start request. |
| `procedure.directory` | Procedure directory from the start request. |
| `summary.activity_count` | Number of activities in the report. |
| `summary.action_count` | Number of actions in the report. |
| `summary.actions_run` | Current count of actions with `pass`, `fail`, or `inconclusive` outcomes. Error outcomes are included in `action_count` but not in `actions_run`. |
| `summary.pass` | Count of passing actions. |
| `summary.fail` | Count of failing actions. |
| `summary.inconclusive` | Count of inconclusive actions. |
| `summary.outcome` | Overall report outcome. |
| `activity[].name` | Activity name from the procedure. |
| `activity[].action[]` | Evaluated action results for the activity. |
| `action[].name` | Action name from the procedure. |
| `action[].outcome` | Evaluator outcome. |
| `action[].reason` | Evaluator reason string. |
| `action[].test_file.file` | Procedure-relative test path. |
| `action[].test_file.signature` | SHA256 signature of the test file contents. |
| `action[].evidence_file.file` | Procedure-relative evidence path. |
| `action[].evidence_file.signature` | SHA256 signature of the evidence file contents. |

## Outcome Values

Valid action outcomes:

- `pass`
- `fail`
- `inconclusive`
- `error`

The current summary outcome logic is:

- If any action is `inconclusive` or `error`, the report outcome is `inconclusive`.
- Otherwise, if any action is `fail`, the report outcome is `fail`.
- Otherwise, the report outcome is `pass`.

The current summary shape does not expose a separate `error` count.

## Signatures

NAPE calculates SHA256 signatures for every evaluated test file and evidence file.

Format:

```text
SHA256[<hex-digest>]
```

Example:

```yaml
test_file:
  file: activity/pet-medicine-app/database-connection.py
  signature: SHA256[fa5e8181c6c40cee0e27c037b25208f8c731a25d77d23aac749c3fbc06bba6c0]
```

Signatures allow users to confirm exactly which test and evidence bytes were used for the report.

## Release 1 Example Summary

The current Rover Release 1 local run produced:

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

## Release 3 Example Summary

The current Rover Release 3 local run produced:

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

The four inconclusive actions are currently the unimplemented Rover Cloud and Exa-Doo SOC1 checks.
