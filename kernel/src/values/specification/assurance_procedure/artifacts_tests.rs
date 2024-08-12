
use crate::error::{Audience, Kind};
use crate::values::specification::assurance_procedure::artifact::Artifact;
use crate::values::specification::assurance_procedure::artifacts::Artifacts;


#[test]
fn add_artifact_success() {
    let artifacts = Artifacts::default();
    let updated_artifacts = artifacts.add("artifact-1", "some artifact description", &vec![("key-1".to_string(), "some value".to_string())]).unwrap();
    assert_eq!(updated_artifacts.count(), 1);
}

#[test]
fn test_merge_artifact() {
    let artifacts = Artifacts::default();
    let artifact = Artifact::new("artifact-1", "some description", &vec![("key-1".to_string(), "some value".to_string())]).unwrap();
    let new_artifacts = artifacts.merge(&artifact).unwrap();
    assert_eq!(new_artifacts.count(), 1);
}


#[test]
fn add_artifact_error_already_exists() {
    let artifacts = Artifacts::default();
    let updated_artifacts = artifacts.add("artifact-1", "some description", &vec![("key-1".to_string(), "value-1".to_string())]).unwrap();
    let error = updated_artifacts.add("artifact-1", "description", &vec![("key".to_string(), "value".to_string())]).unwrap_err();

    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert!(error.message.starts_with("The artifact  'artifact-1' cannot be added because it already exists."));
}

#[test]
fn merge_artifact_error_already_exists() {
    let artifacts = Artifacts::default();
    let first_update = artifacts.add("artifact-1", "description", &vec![("key".to_string(), "value".to_string())]).unwrap();

    let artifact_to_merge = Artifact::new("artifact-1", "description", &vec![("key".to_string(), "value".to_string())]).unwrap();
    let second_update_result = first_update.merge(&artifact_to_merge);

    let error = second_update_result.unwrap_err();

    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert!(error.message.starts_with("The artifact 'artifact-1' cannot be added because an artifact with the name 'artifact-1' already exists."));
}