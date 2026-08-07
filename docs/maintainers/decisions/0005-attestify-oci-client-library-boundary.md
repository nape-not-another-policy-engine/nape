# Decision 0005: Attestify OCI Client Library Boundary

Status: accepted.

`attestify-oci-oss` is an external, reusable, domain-neutral client-extension
library for deterministic Attestify package and OCI mechanics. NAPE consumes
exact signed release `0.1.0`, locked to commit
`013bb352366fcad80db682984705911ca2108258`. The library owns stable source
sealing, canonical package construction, Lock mechanics, common verification,
safe materialization, custody, PURL-to-Registry mapping, and its restricted
five-operation OCI transport boundary. It is not a complete OCI client.

NAPE owns its concrete Verification package profile, Product-semantic
definition admission, effective execution occurrences, subjects, evidence,
Evaluator invocation, continuation, Reports, Evidence Set relationships,
command receipts, and current-run state. NAPE drivers call high-level library
operations; they do not reproduce archive, manifest, Lock, mapping, or
transport algorithms. NAPE does not embed or vendor either the library or a
patched `oci-client` source tree.
