# Verification V2 Authoring

Author one closed YAML root per package:

- `verification-action.yaml` for an independent Action;
- `verification-activity.yaml` for an independent Activity; or
- `verification-procedure.yaml` for a Procedure.

All use `apiVersion: 2.0.0`. Action and Activity references contain only
`name` and exact versioned `package`. Embedded definitions contain their own
label, claim, evidence, Test, and optional current V2 functional declarations.

Names used for Activity and Action occurrences follow the hyphen grammar and
form report selectors such as:

```text
release-readiness.database-connection
```

Evaluation subject field identifiers use the current data-field grammar, for
example `database_endpoint`.

Source contains no ULID and no authored Lock. NAPE creates package identities
from the exact PURL and deterministic build bytes. The runtime Verification
Report receives its own ULID at `metadata.id`.

Evidence and Test declarations start with only `name` and `file`. Additional
media type, schema, evaluation, helper-module, and resource declarations add
explicit constraints; they are not inferred. See the maintained
[`minimal` and `complete` examples](../examples/verification-v2-oci/README.md).
