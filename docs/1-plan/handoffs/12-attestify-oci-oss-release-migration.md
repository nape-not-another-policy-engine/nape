# Handoff 12: Attestify OCI OSS Release Migration

Status: **complete; ready to return to Workflow Harness WPC-M1**

## Completed

- corrected OSS commit `013bb352366fcad80db682984705911ca2108258`
  is pushed and released as signed tag `0.1.0`;
- candidate-revision compatibility passed;
- the complete clean exact-release pre-removal NAPE suite passed, including
  both bounded localhost registry tests that previously could not execute; and
- NAPE's manifest and lock resolve exact release `0.1.0` to the corrected
  commit;
- the superseded embedded OCI library and patched OCI-client source are
  removed;
- NAPE's Cargo dependency and Rust imports use the explicit OSS identity
  `attestify-oci-oss` / `attestify_oci_oss`;
- active architecture and structural gates require the exact external OSS
  release and prohibit restoration of the retired paths;
- the complete post-removal in-place suite passed; and
- the independent clean composite suite passed at synthetic proof commit
  `b4a672ea733af2f6f166eba1bab0b726ab43bcd1`.

## Next work

Return to the Workflow Harness WPC-M1 publication-and-consumption sequence.
The next work may consume the exact OSS release through consumer-owned
Workflow package and catalog semantics. It MUST NOT restore Verification,
Workflow, or Risk vocabulary to the domain-neutral OCI library.

## Exact stop

The NAPE migration is closed. Proprietary OCI behavior remains deferred until
a concrete non-NAPE requirement exists. Workflow package/catalog OCI work is
governed separately by WPC-M1.
