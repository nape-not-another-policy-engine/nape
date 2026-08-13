# V1 Invocation Metadata Transition

NAPE V2 retains the V1 capability to supply repeatable flat caller metadata at Start. It does not introduce `invocation.metadata`; caller entries remain in the Verification Report's top-level `metadata` object beside Engine-owned fields.

Each caller entry is a bounded key/value text pair. Reserved Product and Engine keys, including `id`, `generated_at`, and `utc-start`, cannot be supplied by the caller. Duplicate keys, invalid names, control characters, and exceeded count/byte limits fail Start. The complete admitted map is frozen in current-run state and reproduced unchanged in the Report. NAPE generates `utc-start`, Report `id`, and `generated_at`.

This transition preserves the user capability but does not restore AssuranceProcedure or AssuranceReport execution.
