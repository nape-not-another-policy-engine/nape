# ADR 0001: Runtime State File

Status: current V1 behavior

## Context

`nape collect start` creates a run workspace and writes active run state to:

```text
$HOME/nape/.nape_cli_config
```

`nape collect evidence` and `nape collect report` read that state to find the active run.

## Decision

V1 tracks one active run per `HOME` value.

## Consequences

Positive:

- Commands after `collect start` do not need a run ID argument.
- The CLI is simple for one-run workflows.
- Tests and examples can isolate state by setting `HOME`.

Negative:

- Concurrent runs require isolated `HOME` values or careful sequencing.
- Starting a new run overwrites the active state pointer.
- Users must inspect files to understand which run is active.

## V2 Considerations

V2 should decide whether to preserve this simple state model or introduce explicit run IDs, project-local state, or multiple active runs.

