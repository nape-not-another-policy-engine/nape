# Attestify Python Test Development V2 Runner Profile

## Classification

This document is a normative development Runner Profile specification.

## Status and purpose

The profile identifier is exactly:

```text
attestify-python-test-development-v2
```

It is selected only by
`attestify.nape-evaluator.action-invocation/v3` for the I6 F01-F06 functional
delivery. Development V1 and Evaluator V2 remain unchanged and replayable.

This profile adds only:

- `opaque` and `validated-structured` evidence arguments;
- zero through 64 complete effective evaluations; and
- an exact Action-owned helper-module map.

It is not the production `attestify-python-test-v1` profile and does not claim
enterprise sandbox, hostile-code, scheduler, queue, watchdog, credential,
trust, or network hardening.

## Fixed Test ABI

The sole semantic entry point remains:

```python
def evaluate(evidence, evaluations, metadata):
    ...
```

The Runner calls it exactly once per successfully activated Action occurrence.
There is no fourth argument, wrapper, alternate entry point, overload,
fallback, or metadata-carried parameter channel.

For `opaque`, `evidence` is the immutable admitted bytes value. For
`validated-structured`, it is one fresh occurrence-private JSON-compatible
value produced by NAPE's controlled JSON, YAML, or TOML decoder after schema
validation. The Evaluator does not decode or validate the raw source evidence.

`evaluations` is a fresh occurrence-private list containing the complete
effective evaluation values in exact package/authored order. The Engine and
Runner never alphabetize or otherwise reorder it. It contains no
`criteria_assignment`, criterion origin, package identity, Procedure data, or
fallback value. It may be empty.

`metadata` contains exactly:

```text
evidence_media_type
evidence_representation
evaluator_contract_version: "3"
```

The media type and representation must equal the V3 evidence descriptor.

## Exact helper-module map

The V3 Test descriptor supplies one entry file and zero through 64 modules.
Each module descriptor contains exactly:

```text
name
file
content_digest
byte_count
```

Names are unique ASCII-sorted identifiers matching
`^[a-z][a-z0-9_]*$`. File paths and digests are already verified by NAPE and
must equal the package-owned declaration. The map is the complete reachable
acyclic helper closure.

The Runner prepares one closed direct name-to-bytes mapping. It performs no
filesystem, `sys.path`, environment, site-package, installed-package,
network, OCI, or import-hook search. A package helper cannot collide with an
admitted Runner standard module. The Test entry is not importable as a
helper, and no helper may export or own the semantic `evaluate` result.

Repeated imports within one occurrence return the same occurrence-local
module object. Separate Action occurrences receive fresh realms and fresh
module objects even when their verified source bytes are identical. Teardown
destroys all occurrence-local executable state.

## Admission and activation order

Before any source operation executes, NAPE and the Evaluator together prove:

1. the exact V3 request schema and metadata equality;
2. the entry and module byte counts and digests;
3. the complete module map, namespace disjointness, and static source rules;
4. evidence and evaluation argument admissibility;
5. effective execution and result limits; and
6. a fresh occurrence execution realm.

Mapping preparation creates no module object and runs no top-level source.
Activation begins only after NAPE has admitted all evidence for the complete
invocation.

## Functional execution outcomes

The profile emits the same closed V2 execution-observation vocabulary through
the V3 response discriminator. Valid completed Test results preserve
Test-of-Detail ownership. Invalid results, result-limit failures, blocked
activation, and terminated initialization/evaluation retain their existing
Engine-owned contained semantics. Raw tracebacks, host paths, source text,
credentials, and mutable runtime objects never cross the response boundary.

Controlled conformance observations may prove sandbox, watchdog, capacity,
or worker-loss Engine behavior that the development provider cannot yet
produce through production-grade enforcement. Such proof does not upgrade
this profile's development qualification.

## Exclusions

This profile does not admit Fact Finders, Fact Packets, multiple evidence
arguments, independently resolved OCI libraries, native extensions, dynamic
imports, ambient dependencies, Risk logic, Harness workloads, client code,
or OCI behavior. NAPE Evaluator still receives and executes one prepared Test
occurrence only.

## Machine contracts

The exact request, response, definition, Report, and relationship selectors
are in [`schemas/i0-i6/`](schemas/i0-i6/). The stable diagnostics are in the
current release under
[`schemas/machine-readable-error-registry-v1/`](schemas/machine-readable-error-registry-v1/).
