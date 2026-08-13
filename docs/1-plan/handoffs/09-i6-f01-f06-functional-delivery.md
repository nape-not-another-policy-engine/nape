# Handoff 09: I6-F01 Through I6-F06 Functional Delivery

Status: approved, complete, and frozen; stop before every post-F06 behavior

## Restart order

1. Read `docs/1-plan/roadmap.md`.
2. Read `docs/1-plan/plans/09-i6-f01-f06-functional-delivery.md`.
3. Read the central approved terminal plan and program integration record.
4. Re-ground on architecture, traceability, and ADR 0003.
5. Preserve Plan 08 and I5 as the predecessor baseline.

## Completed state

Do not redesign or reimplement F01-F06. The NAPE functional implementation,
real Evaluator V3 selection, deterministic current source preparation, and
two-clean-registry qualification are complete.

Preserve:

- current Receipt V2 zero-or-more unique Edge rows, exact I6 Report-summary
  capacity, frozen I0-I5 Receipt bytes, and the identity bridge;
- the approved current A7 strict-decode and `.get(...)` source preparation;
- final Action `sha256:1d78c238...965d`, Activity
  `sha256:e1b4c875...6b36`, and Procedure `sha256:e5ede5ec...cd9`;
- Engine projection `sha256:ef966f74...24c9e` and Evaluator V3 projection
  `sha256:8885a6be...69deb`;
- exact `parent-assignment` Report vocabulary; and
- the fixed Test ABI, NAPE V1, Evaluator V2, and I0-I5 behavior.

Final proof:

- two clean registry runs pass with seven effective occurrences and equal
  normalized semantic results;
- NAPE passes 187 tests;
- Evaluator passes 194 tests with ten expected exact-host skips;
- Product current and predecessor validators pass; and
- both installed projections verify independently.

The local exact-profile wrapper is
`tools/i6-nape-eval-container-wrapper.sh`. Set `TMPDIR=/private/tmp` when using
it so every NAPE workspace path is visible through the wrapper's exact mount.

## Next boundary

Stop before F07 Fact Finding and the full Attestify Risk Engine plan. Also do
not begin outcome persistence/UI, authentication or trust, catalog,
independent libraries, enterprise Runner hardening, OCI result publication,
client, or Harness work without their separate authority.
