# Generated Read-Only Contract Projection

This directory is generated from the authoritative Attestify Product
Specification. It is a repository-local input, not a contract-authoring
location.

Rules:

- do not edit projected files directly;
- runtime contract files and test-only conformance files remain separate;
- verify `projection-pin.json`, `projection-manifest.json`, and every listed
  file before build or test use;
- regenerate only from the authoritative Product Specification source; and
- fail on any missing, additional, changed, renamed, or symbolic-link file.

The projection introduces no runtime dependency on a Product Specification
checkout, network location, Git repository, OCI registry, or neighboring
workspace.
