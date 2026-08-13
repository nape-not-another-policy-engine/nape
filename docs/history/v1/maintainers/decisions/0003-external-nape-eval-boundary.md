# ADR 0003: External NAPE Evaluator Boundary

Status: current V1 behavior

## Context

`nape collect report` shells out to `nape-eval`.

Before evaluating actions, the adapter runs:

```bash
nape-eval --check-install
```

For each evidence/test pair, it runs:

```bash
nape-eval --evidence <evidence-file> --test <test-file>
```

## Decision

V1 keeps test execution outside the Rust CLI and uses `nape-eval` as a separate runtime dependency.

## Consequences

Positive:

- Test execution can use Python test files.
- The Rust CLI stays focused on orchestration, state, signing, and report generation.
- Evaluator implementation can evolve separately from the CLI.

Negative:

- Users must install and verify two CLIs.
- Environment failures prevent report generation.
- Non-zero evaluator execution currently behaves differently from an evaluator-returned `error` outcome.

## V2 Considerations

V2 should decide whether evaluator environment failures produce report diagnostics, action-level errors, or fatal command failures. V2 should also decide whether the evaluator contract needs a versioned JSON schema.

