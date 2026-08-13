# Decision 0003: External NAPE Evaluator Boundary

Status: accepted.

NAPE invokes the Evaluator once for each effective Action occurrence. The request contains one prepared Python Test of Detail, one isolated admitted evidence argument, effective evaluations, Test metadata, resource limits, and one exact Runner Profile. The fixed Test ABI is `evaluate(evidence, evaluations, metadata)`.

The Evaluator returns execution state, `true`/`false`/`inconclusive`, facts, and reason. It does not receive or interpret the Procedure, Activity, Action definition, PURL, Registry location, manifest, Lock, provenance, continuation policy, Report identity, or OCI trust data. V2 is the opaque/minimal boundary; V3 adds functional evaluation/module data without changing the Test ABI.
