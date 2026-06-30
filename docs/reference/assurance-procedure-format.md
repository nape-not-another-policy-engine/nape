# Assurance Procedure Format

An assurance procedure defines what NAPE should evaluate. It names the subject-level procedure, groups expectations into activities, and maps each action to a test file and an evidence file.

The current procedure format is YAML.

## Required Repository Layout

A procedure directory must contain:

```text
assurance_procedure.yaml
activity/
```

Example:

```text
rover-mediical-system/release-3/
  assurance_procedure.yaml
  activity/
    pet-medicine-app/
      database-connection.py
```

During `nape collect start`, NAPE copies these into the run workspace:

```text
<run-home>/
  assurance_procedure.yaml
  activity/
```

## Top-Level YAML Shape

```yaml
apiVersion: 1.0.0
kind: AssuranceProcedure
procedure:
  nrn: "nrn:procedure:rover-medical/rover-medicine-system"
  short: "Rover Medical System - Verification Procedure"
  description: |
    "Verifies the configuration and communication expectations."
activity:
  - name: pet-medicine-app
    short: "Pet Medicine Application Configuration"
    description: |
      "Verify the Pet Medicine Application configuration."
    action:
      - name: database-connection
        short: "Database Connection String"
        description: "The application connects to the expected database."
        test: "activity/pet-medicine-app/database-connection.py"
        evidence: "evidence/pet-medicine-app/app-config.toml"
```

## Fields

| Field | Required | Description |
| --- | --- | --- |
| `apiVersion` | Yes | Semantic version string such as `1.0.0`. |
| `kind` | Yes | Must identify this file as an assurance procedure. Current value is `AssuranceProcedure`. |
| `procedure.nrn` | Yes | NRN for the procedure. |
| `procedure.short` | Yes | Short human-readable procedure label. |
| `procedure.description` | Yes | Longer procedure description. |
| `activity` | Yes | List of activities. |
| `activity[].name` | Yes | Activity name. Must match NAPE name rules. |
| `activity[].short` | Yes | Short activity label. |
| `activity[].description` | Yes | Longer activity description. |
| `activity[].action` | Yes | List of actions. |
| `action[].name` | Yes | Action name. Must match NAPE name rules. |
| `action[].short` | Yes | Short action label. |
| `action[].description` | Yes | Longer action description. |
| `action[].test` | Yes | Test file path relative to the run workspace. |
| `action[].evidence` | Yes | Evidence file path relative to the run workspace. |

## Compatibility Note: `expect-evidence`

Some older or external example YAML may include an `expect-evidence` field under an activity.

The current v1 serializer used by this repository models activities with:

- `name`
- `short`
- `description`
- `action`

It does not model `expect-evidence` as a required field. Treat action-level `evidence` paths as the current source of truth for NAPE runs.

## Name Rules

Activity and action names use the NAPE name contract:

- Non-empty
- Alphanumeric and dashes only
- Cannot start or end with a dash
- Stored lowercase by the value object

Valid:

```text
pet-medicine-app
database-connection
exa-doo-dc
```

Invalid:

```text
pet_medicine_app
pet medicine app
-database-connection
database-connection-
```

## Path Rules

`action[].test` and `action[].evidence` should be paths relative to the run workspace.

Given:

```yaml
test: "activity/pet-medicine-app/database-connection.py"
evidence: "evidence/pet-medicine-app/app-config.toml"
```

NAPE expects these files to exist after evidence collection:

```text
<run-home>/activity/pet-medicine-app/database-connection.py
<run-home>/evidence/pet-medicine-app/app-config.toml
```

## Test File Contract

Current Rover examples use Python tests evaluated by `nape-eval`.

A test exposes an `evaluate` function and returns:

```python
return "pass", "Reason explaining the result."
```

Valid outcome strings:

- `pass`
- `fail`
- `inconclusive`
- `error`

The evaluator accepts outcome values case-insensitively, but generated reports serialize lowercase outcomes.

## Rover Release Examples

Release 1 has one activity and one action:

```yaml
activity:
  - name: pet-medicine-app
    action:
      - name: database-connection
        test: "activity/pet-medicine-app/database-connection.py"
        evidence: "evidence/pet-medicine-app/app-config.toml"
```

Release 3 expands to:

- `pet-medicine-app`
- `pet-medicine-db`
- `pet-medicine-host`
- `rover-cloud`
- `exa-doo-dc`

The current Release 3 report has 22 actions.
