# Composed Quality Assessment

Status before C6: pending live closure.

The isolated assessment requires formatting, all-target compilation, strict Clippy, strict Rustdoc, unit/integration tests, dependency/ownership checks, documentation/diagram checks, Product projection validation, and controlled local lifecycle smoke. The composed assessment adds two clean Registry lifecycles and real Evaluator V2/V3 processes.

A module is `pass` only when its local checklist and every applicable composed proof pass. It is `inconclusive` when a declared external live prerequisite is unavailable. It is `fail` for a reproducible contract, safety, integrity, or behavior violation. Inconclusive live evidence cannot be used to claim release closure.

The final C6 record must list exact commands, controlled inputs, results, identities, nondeterministic comparison rules, and any non-applicable security/performance fields.
