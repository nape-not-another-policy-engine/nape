# Module Quality Specifications

Every bounded module is assessed with the same fields: identity; responsibility; module type; inputs; outputs; failure outputs; logical paths; hostile/limit edges; determinism; seam assumptions; focused verification; safety; security applicability; performance applicability; and a pass/fail/inconclusive result.

## Domain modules

The individually addressable set is `diagnostic`; every `value/*`; every `gateway/*`; every `service/*`; and every `usecase/*` listed in Plan 10 Section 5.1. Values admit bounded transport-neutral state. Gateway modules define one capability only. Services are deterministic and side-effect free. Use Cases orchestrate explicit Gateways. Pass requires only `kernel_oss` in production, sibling tests, no parser/serializer/transport/process/filesystem/environment/time/randomness dependency, and every logical path in the manifest.

## Application modules

The set is `composition`, every `gateway_driver/*`, and every `io_adapter/clap/*` listed in Plan 10 Section 5.2. Composition owns runtime/configuration/injection. Drivers translate one Gateway to one external capability. Adapters own grammar, wire projection, stdout, and exit decisions. Pass requires no duplicated package algorithm, no hidden service location, bounded stable I/O, one diagnostic translation, sibling tests, and strict warning-free builds.

## Attestify OCI modules

The set is `error`, `source`, `package`, `custody`, `registry`, and `registry_client` listed in Plan 10 Section 5.3. They own reusable package/profile behavior, restartable custody, deterministic mapping, and the internal five-operation transport. Pass requires no NAPE semantic type, no evidence/Evaluator/Report/continuation behavior, no hidden runtime, exact digest/size/media checks, safe path handling, and deterministic common-verifier parity.

## Assessment rule

For each module, inputs and outputs are the public values in its `mod.rs`; failure outputs are its bounded diagnostic/Kernel error contract; logical and hostile paths come from the [logical-path manifest](../reference/engineering-standards-logical-paths.md); focused verification is its sibling `tests.rs`; seam assumptions are the [Gateway inventory](../maintainers/gateway-contracts.md); security and performance are required when the module crosses files, processes, archives, registries, or ceilings. A missing required field or failed applicable proof is `fail`; an unavailable declared live prerequisite is `inconclusive`; otherwise the module is `pass`.
