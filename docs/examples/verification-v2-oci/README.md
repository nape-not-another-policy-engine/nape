# Verification V2 OCI Lifecycle Examples

These are checked, directly buildable NAPE CLI 2.0 examples. They contain no
authored Lock and require no historical-source translation.

- `minimal/` is one Procedure with one embedded Activity and Action.
- `complete/` is one independently built Action, one independently built
  Activity containing one embedded Action plus the referenced Action, and one
  root Procedure referencing the Activity.
- `registry-map.yaml` is the exact-publisher configuration for a registry
  exposed at `http://localhost:5001`.

The examples use anonymous localhost HTTP because authentication and trust are
outside the current development profile. A port forward can expose a remote
development registry at the configured localhost port.

## Minimal local lifecycle

From this directory:

```bash
nape package build \
  --source ./minimal/source \
  --package pkg:attestify/acme.example/verification-procedure/release-readiness/database-endpoint@1.0.0 \
  --output ./build/minimal-procedure

nape start \
  --local-package ./build/minimal-procedure \
  --subject-file ./minimal/subject.json

nape evidence \
  --action release-readiness.database-connection \
  --file ./minimal/evidence/release-readiness/database-connection/application-configuration.json

nape verify
```

## Complete build and publish lifecycle

Build leaf packages first and supply the complete verified dependency set to
the root build:

```bash
nape package build \
  --source ./complete/source/action \
  --package pkg:attestify/acme.example/verification-action/release-readiness/database-connection@2.0.0 \
  --output ./build/action

nape package build \
  --source ./complete/source/activity \
  --package pkg:attestify/acme.example/verification-activity/release-readiness/database-readiness@2.0.0 \
  --dependency-package ./build/action \
  --output ./build/activity

nape package build \
  --source ./complete/source/procedure \
  --package pkg:attestify/acme.example/verification-procedure/release-readiness/release-readiness@2.0.0 \
  --dependency-package ./build/action \
  --dependency-package ./build/activity \
  --output ./build/procedure
```

Publish the exact accepted build outputs with one configured endpoint:

```bash
nape package publish --local-package ./build/action --registry-endpoint http://localhost:5001
nape package publish --local-package ./build/activity --registry-endpoint http://localhost:5001
nape package publish --local-package ./build/procedure --registry-endpoint http://localhost:5001
```

Each command emits exactly one Receipt V2 JSON line. Record the root
`manifestDigest` from the Procedure publish receipt. Resolve and execute that
exact release:

```bash
nape package resolve \
  --package pkg:attestify/acme.example/verification-procedure/release-readiness/release-readiness@2.0.0 \
  --manifest-digest sha256:<procedure-manifest-digest> \
  --registry-endpoint http://localhost:5001 \
  --plan-only

nape start \
  --package pkg:attestify/acme.example/verification-procedure/release-readiness/release-readiness@2.0.0 \
  --manifest-digest sha256:<procedure-manifest-digest> \
  --registry-endpoint http://localhost:5001 \
  --subject-file ./complete/subject.json

nape evidence \
  --action infrastructure-release-readiness.configuration-format \
  --file ./complete/evidence/infrastructure-release-readiness/configuration-format/application-configuration.json

nape evidence \
  --action infrastructure-release-readiness.database-connection \
  --file ./complete/evidence/infrastructure-release-readiness/database-connection/application-configuration.json

nape verify
```

`--registry-endpoint` and `--registry-profile` are mutually exclusive. The
single endpoint maps every canonical `pkg:attestify` publisher in the exact
Lock closure to that endpoint under the fixed `attestify/` repository prefix.
The full Registry Map remains available when publishers require different
exact mappings.

Start and Evidence success are silent. Verify emits one Receipt V2 whose
`result.output` identifies the NAPE-managed immutable result directory.
Successful verification atomically creates:

```text
<nape-managed-result-directory>/
├── verification-report.json
├── evidence-set-relationship.json
└── evidence/sha256/<digest>
```

It does not publish the Verification Report to OCI.

## Automated lifecycle proof

With a clean anonymous registry listening on a free localhost port and the
exact Python 3.11.6 Evaluator wrapper configured:

```bash
export ATTESTIFY_WORKSPACE_ROOT=/absolute/path/to/1-attestify-product-workspace
export NAPE_V2_EVALUATOR="$ATTESTIFY_WORKSPACE_ROOT/applications/nape/tools/i6-nape-eval-container-wrapper.sh"

python tools/verification_v2_lifecycle_smoke.py \
  --registry-endpoint http://localhost:51247
```

The proof uses `/private/tmp` by default so the containerized Evaluator sees
the exact same absolute workspace path. Override that only with an existing
absolute `NAPE_V2_TEMP_ROOT` mounted at the identical path. The proof requires
a clean registry and does not create or delete the registry process.
