# ADR 0002: Procedure Retrieval Boundary

Status: current V1 behavior

## Context

`nape collect start` retrieves a procedure directory from a user-supplied procedure link and directory.

Current supported procedure link schemes are:

- `file`
- `git`
- `https`

`ssh://` links are rejected by the current repository link value contract.

## Decision

V1 separates procedure retrieval behind gateway adapters. Local filesystem retrieval and Git-backed retrieval are selected from the procedure link scheme.

## Consequences

Positive:

- Local `file://` runs support deterministic examples and offline procedure development.
- Git-backed retrieval allows remote procedure catalogs.
- Retrieval is isolated from the core start-collection use case.

Negative:

- Current reports record the repository link and directory, but not a resolved commit.
- SSH users must clone locally and use `file://`.
- Remote repository drift can change future runs unless the caller pins the source externally.

## V2 Considerations

V2 should decide whether to support SSH links, record resolved commits, require immutable procedure references, or add first-class catalog metadata.

