# Composition Root

`NapeApplication` is the only process composition root. It reads bounded process configuration, creates concrete Gateway drivers, and injects those drivers into one of the six Domain Use Cases. Use Cases never locate services or read environment variables.

The Tokio runtime is created by the executable entry point.
`attestify-oci-oss` exposes async operations but never constructs a hidden
runtime. NAPE supplies its exact Verification package profile to the library.
Registry configuration is command-owned and projected to one `RegistryMap`.
Evaluator V2/V3 executable paths, build records, and expected record digests
are selected from complete environment triples; partial configuration fails
before execution.

Package bytes stay in Application-private custody. Domain receives transport-neutral verified observations, controlled roots, exact file bytes, independently observed file digests, and opaque handles. Start commits restartable custody; Evidence and Verify reopen it without network access.

![Composition wiring](../assets/diagrams/plantuml/rendered/02-composition-wiring.svg)

Source: [02-composition-wiring.puml](../assets/diagrams/plantuml/source/02-composition-wiring.puml)
