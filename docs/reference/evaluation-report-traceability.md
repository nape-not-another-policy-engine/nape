# Verification Evaluation And Report Traceability

Each Report Action row traces one contextual `activity.action` occurrence to:

- the exact Procedure, Activity, and Action package owners and manifest digests;
- embedded/package origin where that distinction affects byte ownership;
- the definition-owned Test path, digest, modules, Runner Profile, and implementation observations;
- the definition-owned Evidence declaration, occurrence association, digest, byte count, representation, and optional schema validation;
- effective Evaluation criteria and each criterion's package-default or parent-assignment origin;
- Evaluator execution state, conclusion, facts, reason, and diagnostic; and
- the Report summary and Evidence Set relationship.

The Evidence Set relationship records canonical occurrence paths and digest-addressed payload relationships, never host-local physical paths. The local output contains `verification-report.json`, `evidence-set-relationship.json`, and exact raw evidence under `evidence/sha256/<digest>` committed by one sibling same-filesystem rename.
