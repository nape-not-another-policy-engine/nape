
use crate::error::{Audience, Kind};
use crate::values::specification::assurance_procedure::artifact::Artifact;

#[test]
fn new_artifact_success() {
    let artifact = Artifact::new("artifact", "artifact description", &vec![("artifact-md-1".to_string(), "some description".to_string())]);
    assert!(artifact.is_ok());
}

#[test]
fn new_artifact_error_invalid_name() {
    let artifact = Artifact::new("bad artifact name", "artifact description", &vec![("artifact-md-1".to_string(), "some description".to_string())]);

    assert!(artifact.is_err());

    let error = artifact.unwrap_err();
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert!(error.message.starts_with("There is an issue with your artifact name 'bad artifact name': "));
}

#[test]
fn test_new_artifact_error_invalid_description() {
    let artifact = Artifact::new("artifact-1", "", &vec![("artifact-md-1".to_string(), "some description".to_string())]);
    assert!(artifact.is_err());
    let error = artifact.unwrap_err();
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert!(error.message.starts_with("There is an issue with your artifact description '': "));
}

#[test]
fn test_new_artifact_error_invalid_metadata() {
    let artifact = Artifact::new("artifact-1", "This is an artifact", &vec![("bad key name".to_string(), "some description".to_string())]);
    assert!(artifact.is_err());
    let error = artifact.unwrap_err();
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert!(error.message.starts_with("There is an issue with your artifact metadata 'bad key name': "));
}