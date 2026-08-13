# Procedure Authoring

This guide explains how to author a V1 NAPE assurance procedure that works with the current CLI.

Use this when you need to create or update `assurance_procedure.yaml`, add tests of detail, or prepare evidence collection commands.

## Authoring Model

Think in four layers:

- Subject: the thing being evaluated.
- Procedure: the full set of verification expectations.
- Activity: a group of related actions and evidence files.
- Action: one testable expectation that maps one test file to one evidence file.

Current evidence collection is activity-oriented. Users pass an activity name to `--control-activity`, and NAPE copies evidence under:

```text
<run-home>/evidence/<activity-name>/
```

Actions then reference evidence files under that activity directory.

## Directory Layout

A procedure directory must contain:

```text
assurance_procedure.yaml
activity/
```

Recommended layout:

```text
my-procedure/
  assurance_procedure.yaml
  activity/
    app-config/
      database-connection.py
      tls-enabled.py
```

## Minimal Procedure

```yaml
apiVersion: 1.0.0
kind: AssuranceProcedure
procedure:
  nrn: "nrn:procedure:example/my-system"
  short: "Example System Verification"
  description: "Verifies example system configuration."
activity:
  - name: app-config
    short: "Application Configuration"
    description: "Verify application configuration evidence."
    action:
      - name: database-connection
        short: "Database Connection"
        description: "The application connects to the expected database host."
        test: "activity/app-config/database-connection.py"
        evidence: "evidence/app-config/app-config.toml"
```

## Name Rules

Use NAPE names for activities and actions:

- Non-empty
- Alphanumeric and dashes only
- Cannot start with a dash
- Cannot end with a dash
- Stored lowercase by the value object

Good:

```text
app-config
database-connection
soc1-within-expiry
```

Bad:

```text
app_config
app config
-database-connection
database-connection-
```

## Evidence Path Rules

Use workspace-relative evidence paths:

```yaml
evidence: "evidence/app-config/app-config.toml"
```

Then collect evidence with the matching activity name:

```bash
nape collect evidence \
  --control-activity "app-config" \
  --file-path "./generated/app-config.toml" \
  --file-name "app-config.toml"
```

This creates:

```text
<run-home>/evidence/app-config/app-config.toml
```

Avoid authoring procedure paths with:

- absolute paths
- `../`
- paths outside the run workspace
- activity names that do not match the `--control-activity` value users will pass

## Test Path Rules

Use workspace-relative test paths:

```yaml
test: "activity/app-config/database-connection.py"
```

The test file must be present under the procedure directory's `activity/` tree. During `nape collect start`, NAPE copies `activity/` into the run workspace.

## Test Function Contract

Current examples use Python tests evaluated by `nape-eval`.

A test returns an outcome and reason:

```python
def evaluate(evidence):
    return "pass", "The expected condition is present."
```

Valid outcomes:

- `pass`
- `fail`
- `inconclusive`
- `error`

Use `inconclusive` when the test cannot prove pass or fail from available evidence.

Use `error` only when the test itself can produce a controlled action-level error outcome. A non-zero `nape-eval` command failure currently prevents report generation rather than producing a report action.

## Shared Evidence

Multiple actions can reference the same evidence file.

Example:

```yaml
action:
  - name: database-user
    test: "activity/app-config/database-user.py"
    evidence: "evidence/app-config/app-config.toml"
  - name: database-user-secret
    test: "activity/app-config/database-user-secret.py"
    evidence: "evidence/app-config/app-config.toml"
```

This is useful when one evidence file contains multiple facts and each action tests one expectation.

## Local Smoke Workflow

1. Put the procedure in a local catalog directory.
2. Start a run with `file://` so no remote repository access is required.
3. Collect one evidence file for each expected evidence path.
4. Run `nape collect report`.
5. Inspect `assurance_report.yaml`.

Example:

```bash
nape collect start \
  --subject "nrn:procedure:example/my-system" \
  --subject-id "localtest1" \
  --procedure-link "file:///tmp/my-procedure-catalog" \
  --procedure-directory "my-procedure" \
  --meta system-owner "Example Owner"

nape collect evidence \
  --control-activity "app-config" \
  --file-path "./fixtures/app-config.toml" \
  --file-name "app-config.toml"

nape collect report
```

## Common Authoring Mistakes

| Mistake | Symptom | Fix |
| --- | --- | --- |
| Activity name contains underscore | `collect evidence` or procedure parsing fails validation. | Use dashes, such as `app-config`. |
| `--control-activity` uses action name | Evidence is copied to the wrong directory and report generation cannot find expected evidence. | Pass the activity name. |
| Evidence file name does not match procedure path | Report generation cannot read or sign the evidence file. | Use `--file-name` to match the procedure path. |
| Test path points outside `activity/` | Report generation cannot find the test file. | Keep tests under the procedure `activity/` directory. |
| Procedure uses `ssh://` link | `collect start` rejects the procedure link. | Clone locally with SSH if needed, then use `file://` for NAPE. |
| Metadata omitted | Current handler may fail even though help implies metadata is optional. | Include at least one `--meta` pair. |

## V2 Authoring Notes

V2 should preserve V1 authoring compatibility where practical, but breaking changes are allowed. Any V2 change to path rules, outcome values, metadata behavior, or procedure versioning should include migration guidance from this V1 authoring model.
