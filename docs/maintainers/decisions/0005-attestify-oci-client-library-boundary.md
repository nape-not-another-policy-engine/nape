# Decision 0005: Attestify OCI Client Library Boundary

Status: accepted.

`attestify-oci-oss` is an external, reusable, domain-neutral client-extension
library for deterministic Attestify package and OCI mechanics. NAPE consumes
exact signed release `0.1.1`, locked to commit
`64358b44b8a76b2abfd65f3ad1e043d615d4b6f6`. The library owns stable source
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

As an OSS consumer, NAPE uses the exact Cargo package identity
`attestify-oci-oss` and Rust namespace `attestify_oci_oss`. It does not alias
that dependency through the proprietary `attestify_oci` namespace. The module
ontology beneath both roots remains aligned so the proprietary facade can
proxy the OSS surface without changing conceptual placement.
