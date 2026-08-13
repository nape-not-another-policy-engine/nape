# Engineering Standards Logical Paths

This manifest assigns current behavior to one owner and one proof class.

| Owner | Required logical paths | Principal validation |
| --- | --- | --- |
| Definition admission | Procedure, Activity, Action; embedded/reference; defaults/assignments; unknown/invalid fields | Domain service and build integration tests |
| Effective graph | embedded, referenced Action, referenced Activity, alias reuse, Lock mismatch, limits | Domain service and staged lifecycle tests |
| Build | leaf/nonleaf, exact dependencies, source mutation, path mismatch, existing output | package driver and command integration |
| Publish | exact bytes, map rejection, transport failure, clean re-pull mismatch | publication driver and live lifecycle |
| Resolve plan | root-only, full closure, graph conflict/cycle, digest/media mutation | resolution driver and command integration |
| Start | local/OCI, subject/metadata rejection, atomic non-promotion, new-run selection | Start Use Case, state driver, staged lifecycle |
| Evidence | add, idempotent repeat, replacement, shared payload, wrong assertion, unstable/oversize | Evidence Use Case, commit driver, staged lifecycle |
| Verify | readiness, V2/V3, each conclusion, contained/terminal, hostile response, output non-promotion | Verify Use Case, Evaluator driver, lifecycle |
| Report | origin, criteria, execution, summary, subject/metadata, relationship equality | report service and Product projection |
| Receipt | success/failure, strict JCS/LF/ceiling, exit 0/1/2, Start/Evidence silence | receipt and command-contract tests |
| Custody | materialize/reopen, token confinement, byte mutation, no neighboring discovery | `attestify-oci` custody tests |
| OCI transport | five operations, exact reference, timeout/ceiling, safe errors, localhost policy | Registry-client tests and live qualification |

Every current Rust test identifies a requirement validation point. The machine C0 ledgers remain the historical inventory; current acceptance is `make standards-check`, with live infrastructure isolated under `make qualification-live`.
