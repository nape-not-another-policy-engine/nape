use super::contract_v2::{EvaluatorLimits, EvidenceDescriptor};
use super::contract_v3::*;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

fn request() -> ActionInvocationRequestV3 {
    ActionInvocationRequestV3 {
        contract: CONTRACT.to_string(),
        workspace_root: "/tmp/nape-v3".to_string(),
        evidence: EvidenceDescriptor {
            file: "prepared/0/evidence.json".to_string(),
            argument_digest:
                "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                    .to_string(),
            argument_byte_count: 2,
            media_type: "application/json".to_string(),
            representation: "validated-structured".to_string(),
        },
        test: TestDescriptorV3 {
            file: "prepared/0/test.py".to_string(),
            content_digest:
                "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                    .to_string(),
            byte_count: 10,
            runner_profile: RUNNER_PROFILE.to_string(),
            module: vec![TestModuleDescriptor {
                name: "database_facts".to_string(),
                file: "prepared/0/module/database_facts.py".to_string(),
                content_digest:
                    "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
                        .to_string(),
                byte_count: 10,
            }],
        },
        evaluations: vec![json!({
            "criteria": {"equals": "approved"},
            "label": "Endpoint",
            "name": "endpoint-equals",
            "subject": {"data_type": "text", "label": "Endpoint", "name": "endpoint"}
        })],
        metadata: EvaluatorMetadataV3 {
            evidence_media_type: "application/json".to_string(),
            evidence_representation: "validated-structured".to_string(),
            evaluator_contract_version: "3".to_string(),
        },
        limits: EvaluatorLimits {
            maximum_execution_work_units: 25_000_000,
            maximum_result_bytes: 131_072,
        },
    }
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn admits_closed_functional_request_success() {
    request()
        .validate()
        .expect("functional request should admit");
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn rejects_unsorted_modules_and_incomplete_evaluations_error() {
    let mut value = request();
    value.test.module.insert(
        0,
        TestModuleDescriptor {
            name: "z_helper".to_string(),
            file: "prepared/0/module/z_helper.py".to_string(),
            content_digest:
                "sha256:dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
                    .to_string(),
            byte_count: 10,
        },
    );
    assert!(value.validate().is_err());
    let mut value = request();
    value.evaluations[0] = json!({"name": "endpoint-equals", "criteria": {"equals": "approved"}});
    assert!(value.validate().is_err());
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn preserves_v2_test_abi_while_expanding_only_boundary_data_success() {
    let value = serde_json::to_value(request()).expect("request serialization");
    assert_eq!(value["contract"], CONTRACT);
    assert_eq!(value["metadata"]["evaluator_contract_version"], "3");
    assert_eq!(value["test"]["runner_profile"], RUNNER_PROFILE);
    assert_eq!(value["evaluations"].as_array().map(Vec::len), Some(1));
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn enforces_i6_names_typed_criteria_and_optional_prose_success() {
    let mut value = request();
    value.evaluations[0]["name"] = json!("3-endpoint-2");
    value.evaluations[0]["example"] = json!("A concrete endpoint check.");
    value
        .validate()
        .expect("digit-boundary I6 name should admit");

    let mut wrong_type = value.clone();
    wrong_type.evaluations[0]["criteria"]["equals"] = json!(3);
    assert!(wrong_type.validate().is_err());

    let mut empty_example = value.clone();
    empty_example.evaluations[0]["example"] = json!("");
    assert!(empty_example.validate().is_err());

    let mut empty_array = value;
    empty_array.evaluations[0]["criteria"] = json!({"allowed_values": []});
    assert!(empty_array.validate().is_err());
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn rejects_module_namespace_and_workspace_path_collisions_error() {
    let mut reserved = request();
    reserved.test.module[0].name = "json".to_string();
    assert!(reserved.validate().is_err());

    let mut collision = request();
    collision.test.module[0].file = collision.test.file.clone();
    assert!(collision.validate().is_err());
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn all_contained_and_completed_v2_semantics_admit_under_v3_discriminator_success() {
    let names = [
        "eval-response-v2-test-true.json",
        "eval-response-v2-test-false.json",
        "eval-response-v2-test-inconclusive.json",
        "eval-response-v2-invalid-completed.json",
        "eval-response-v2-over-limit.json",
        "eval-response-v2-blocked-activation.json",
        "eval-response-v2-terminated-initialization.json",
        "eval-response-v2-terminated-evaluate.json",
        "eval-response-v2-request-rejected.json",
        "eval-response-v2-integrity-failed.json",
    ];
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../contracts/attestify-nape-evaluator-action-invocation-v2/conformance/vectors");
    for name in names {
        let mut value: serde_json::Value =
            serde_json::from_slice(&fs::read(root.join(name)).expect("test setup should succeed"))
                .expect("test setup should succeed");
        value["contract"] = json!(CONTRACT);
        validate_response(&value).unwrap_or_else(|error| panic!("{name}: {error:?}"));
    }
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn admits_current_v3_runner_unsupported_phase_without_widening_v2_success() {
    let current = json!({
        "contract": CONTRACT,
        "diagnostic": {
            "code": "runner_profile_unsupported",
            "phase": "runner-call-validation",
            "reason_template": "runner-profile-unsupported-v1",
        },
        "disposition": "request-rejected",
    });
    validate_response(&current).expect("current V3 Source Binding must admit");

    let mut historical_phase = current;
    historical_phase["diagnostic"]["phase"] = json!("runner-profile-selection");
    assert!(validate_response(&historical_phase).is_err());
}
