use attestify_oci::registry::RegistryLocation;
use clap::{Arg, Command};
use nape_domain::gateway::definition_package_publication::RegistryLocationObservation;
use serde_json::{json, Value};

use super::{digest_location_value, location_observation, required};

fn receipt_schema() -> (Value, jsonschema::JSONSchema) {
    let mut schema: Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../contracts/attestify-nape-verification-engine-contract-projection-v2/command-receipt/v2/nape-command-receipt.schema.json"
    )))
    .expect("test setup should succeed");
    let root = schema.as_object_mut().expect("test setup should succeed");
    root.remove("not");
    root.insert("$ref".to_string(), json!("#/$defs/napeCommandReceiptV2"));
    let validator = jsonschema::JSONSchema::options()
        .with_draft(jsonschema::Draft::Draft202012)
        .compile(&schema)
        .expect("test setup should succeed");
    (schema, validator)
}

fn producer() -> Value {
    json!({
        "buildDigest": format!("sha256:{}", "a".repeat(64)),
        "name": "nape",
        "qualification": "attestify-verification-engine-v2-development",
        "version": "2.0.0",
    })
}

fn validation_errors(validator: &jsonschema::JSONSchema, value: &Value) -> String {
    validator
        .validate(value)
        .map_or_else(
            |errors| errors.map(|error| error.to_string()).collect::<Vec<_>>(),
            |()| Vec::new(),
        )
        .join("; ")
}

fn published_location(reference_class: &str) -> RegistryLocationObservation {
    RegistryLocationObservation {
        profile_version: "attestify-oci-registry-map/1".to_string(),
        publisher: "acme.example".to_string(),
        scheme: "http".to_string(),
        registry: "localhost:5000".to_string(),
        repository:
            "attestify/acme.example/verification-procedure/release/readiness/database-endpoint"
                .to_string(),
        reference: "1.0.0".to_string(),
        reference_class: reference_class.to_string(),
    }
}

fn acquired_location(reference_class: &str) -> RegistryLocation {
    let digest = format!("sha256:{}", "b".repeat(64));
    RegistryLocation {
        profile_version: "attestify-oci-registry-map/1".to_string(),
        publisher: "acme.example".to_string(),
        scheme: "http".to_string(),
        registry: "localhost:5000".to_string(),
        repository:
            "attestify/acme.example/verification-procedure/release/readiness/database-endpoint"
                .to_string(),
        publication_tag: "1.0.0".to_string(),
        reference_class: reference_class.to_string(),
        reference: digest.clone(),
        complete_reference: format!("http://localhost:5000/example@{digest}"),
    }
}

/// Requirement validation: the publish projection uses the tag and satisfies Receipt V2.
#[test]
fn package_publish_location_projection_satisfies_receipt_schema_success() {
    let location =
        location_observation(&published_location("tag")).expect("test setup should succeed");
    let receipt = json!({
        "command": "package-publish",
        "contract": "attestify.nape.command-receipt/v2",
        "producer": producer(),
        "result": {
            "disposition": "published",
            "integrity": "digest-verified",
            "location": location,
            "manifestDigest": format!("sha256:{}", "b".repeat(64)),
            "package": "pkg:attestify/acme.example/verification-procedure/release-readiness/database-endpoint@1.0.0",
            "trust": "not-evaluated",
        },
        "status": "succeeded",
    });
    let (_schema, validator) = receipt_schema();
    assert!(
        validator.is_valid(&receipt),
        "{}",
        validation_errors(&validator, &receipt)
    );
}

/// Requirement validation: internal manifest-digest becomes Receipt digest for resolution.
#[test]
fn package_resolve_location_projection_satisfies_receipt_schema_success() {
    let location = digest_location_value(&acquired_location("manifest-digest"))
        .expect("test setup should succeed");
    let receipt = json!({
        "command": "package-resolve-plan",
        "contract": "attestify.nape.command-receipt/v2",
        "producer": producer(),
        "result": {
            "dependency": [],
            "lock": {
                "lockVersion": "1",
                "profileVersion": "attestify-oci-repository-profile/1",
                "validation": "passed",
            },
            "root": {
                "disposition": "verified",
                "integrity": "digest-verified",
                "location": location,
                "manifestDigest": format!("sha256:{}", "b".repeat(64)),
                "package": "pkg:attestify/acme.example/verification-procedure/release-readiness/database-endpoint@1.0.0",
                "trust": "not-evaluated",
            },
        },
        "status": "succeeded",
    });
    let (_schema, validator) = receipt_schema();
    assert!(
        validator.is_valid(&receipt),
        "{}",
        validation_errors(&validator, &receipt)
    );
}

/// Requirement validation: raw internal vocabulary is not a valid Receipt projection.
#[test]
fn raw_manifest_digest_reference_class_fails_receipt_schema_error() {
    let mut location =
        location_observation(&published_location("tag")).expect("test setup should succeed");
    location["referenceClass"] = json!("manifest-digest");
    let receipt = json!({
        "command": "package-publish",
        "contract": "attestify.nape.command-receipt/v2",
        "producer": producer(),
        "result": {
            "disposition": "published",
            "integrity": "digest-verified",
            "location": location,
            "manifestDigest": format!("sha256:{}", "b".repeat(64)),
            "package": "pkg:attestify/acme.example/verification-procedure/release-readiness/database-endpoint@1.0.0",
            "trust": "not-evaluated",
        },
        "status": "succeeded",
    });
    let (_schema, validator) = receipt_schema();
    assert!(!validator.is_valid(&receipt));
}

/// Requirement validation: unknown classes fail before Receipt construction.
#[test]
fn unknown_reference_classes_are_rejected_error() {
    assert!(location_observation(&published_location("unknown")).is_err());
    assert!(digest_location_value(&acquired_location("unknown")).is_err());
    assert!(digest_location_value(&acquired_location("tag")).is_err());
}

/// Requirement validation: the dispatcher reads an already-required Clap value.
#[test]
fn required_argument_projection_is_exact_success() {
    let matches = Command::new("test")
        .arg(Arg::new("value").long("value").required(true))
        .try_get_matches_from(["test", "--value", "expected"])
        .expect("test setup should succeed");
    assert_eq!(required(&matches, "value"), "expected");
}
