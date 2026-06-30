# Example Outputs

This folder contains checked-in examples from validated local Rover Medical NAPE runs.

These examples are documentation artifacts. They are meant to show output shape and expected summary values, not to replace local smoke tests.

## Verified Source Revisions

The examples were generated from these local clones:

| Repository | Commit |
| --- | --- |
| `attestify/rover-medical-system` | `db6702721a5d0d2a56591989647139784829fcc2` |
| `attestify/rover-medical-catalog` | `ba6413314e9ca02198cc9216f60bbc65a58f80c9` |
| `attestify/rover-medical-evidence-collection` | `87be7e5b97dfb78d80be08f99f3401cd365aae5b` |

## Files

`rover-release-1-assurance-report.yaml`

Full generated report from the Release 1 thin-slice run.

`rover-release-3-assurance-report-summary.yaml`

Summary excerpt from the expanded Release 3 run.

`example-nape-cli-config.yaml`

Representative active state file shape with placeholder paths.

`rover-release-3-run-tree.txt`

Representative generated workspace tree for Release 3.

## Expected Summaries

Release 1:

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

Release 3:

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

## Regenerating

Follow:

```text
docs/user/rover-medical-example.md
```

Use an isolated `HOME` when regenerating examples so the run does not overwrite your normal NAPE state.

You can also run the reproducible docs smoke script:

```bash
make docs-rover-smoke
```

This is a local maintainer validation target, not a required CI gate. The script clones the known-good Rover repositories, runs Release 1 and Release 3 in an isolated `HOME`, and validates the expected summary counts plus basic report shape.

Useful overrides:

```bash
NAPE_DOCS_SMOKE_KEEP=1 make docs-rover-smoke
NAPE_BIN=./target/release/nape make docs-rover-smoke
NAPE_DOCS_SMOKE_WORK=/private/tmp/my-nape-smoke make docs-rover-smoke
```

`NAPE_DOCS_SMOKE_WORK` must be under `/tmp` or `/private/tmp` because the script removes and recreates that directory.
