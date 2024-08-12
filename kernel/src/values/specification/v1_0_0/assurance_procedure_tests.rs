use crate::error;
use crate::error::{Audience};
use crate::values::specification::api_version::APIVersion;
use crate::values::specification::kind::Kind;
use crate::values::specification::assurance_procedure::artifact::Artifact;
use crate::values::specification::assurance_procedure::activity::Activity;use crate::values::specification::v1_0_0::assurance_procedure::AssuranceProcedure;

#[test]
fn new_builder() {
    let result = AssuranceProcedure::builder()
        .api_version("1.0.0")
        .procedure_info("nrn:sourcecode::example", "A Short Desc.", "This is an example procedure")
        .add_activity(&Activity::new("procedure-1", "Short Desc", "Long Desc").unwrap())
        .try_build();

    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));

    let result =result.unwrap();
    assert_eq!(result.api_version, APIVersion::from_str("1.0.0").unwrap());
    assert_eq!(result.kind, Kind::AssuranceProcedure);
}

/*** APIVersion Sad Path Tests ***/
#[test]
fn builder_error_missing_api_version() {
    let result = AssuranceProcedure::builder()
        .procedure_info("nrn:sourcecode::example", "A Short Desc.", "This is an example procedure")
        .try_build();

    assert!(result.is_err(), "{}", format!("An was expected by one was not returned: {:?}", result.unwrap() ) );

    let err = result.unwrap_err();
    assert_eq!(err.kind, error::Kind::InvalidInput);
    assert_eq!(err.audience, error::Audience::User);
    assert!(err.message.starts_with("The AssurnaceProcedure could not be created: The APIVersion is required, but was not provided."));
}
#[test]
fn builder_error_invalid_api_version() {
    let result = AssuranceProcedure::builder()
        .api_version("")
        .procedure_info("nrn:sourcecode::example", "A Short Desc.", "This is an example procedure")
        .try_build();

    assert!(result.is_err(), "{}", format!("An was expected by one was not returned: {:?}", result.unwrap() ) );

    let err = result.unwrap_err();
    assert_eq!(err.kind, error::Kind::InvalidInput);
    assert_eq!(err.audience, error::Audience::User);
    assert!(err.message.starts_with("The AssurnaceProcedure could not be created: The APIVersion has an issue: "));
}

/*** Procedure Sad Path Tests ***/
#[test]
fn builder_error_missing_nrn() {
    let result = AssuranceProcedure::builder()
        .api_version("1.0.0")
        .try_build();

    assert!(result.is_err(), "{}", format!("An was expected by one was not returned: {:?}", result.unwrap() ) );

    let err = result.unwrap_err();
    assert_eq!(err.kind, error::Kind::InvalidInput);
    assert_eq!(err.audience, error::Audience::User);
    assert_eq!("The AssurnaceProcedure could not be created: Check that you provided the procedure info. Either the procedure NRN, short description, or long description is missing.", err.message);
}
#[test]
fn builder_error_invalid_procedure_nrn() {
    let result = AssuranceProcedure::builder()
        .api_version("1.0.0")
        .procedure_info("", "A Short Desc.", "This is an example procedure")
        .try_build();

    assert!(result.is_err(), "{}", format!("An was expected by one was not returned: {:?}", result.unwrap() ) );

    let err = result.unwrap_err();
    assert_eq!(err.kind, error::Kind::InvalidInput);
    assert_eq!(err.audience, error::Audience::User);
    assert!(err.message.starts_with("The AssurnaceProcedure could not be created: The procedure information has an issue: "));
}
#[test]
fn builder_error_invalid_procedure_short() {
    let result = AssuranceProcedure::builder()
        .api_version("1.0.0")
        .procedure_info("nrn:sourcecode::example", "", "This is an example procedure")
        .try_build();

    assert!(result.is_err(), "{}", format!("An was expected by one was not returned: {:?}", result.unwrap() ) );

    let err = result.unwrap_err();
    assert_eq!(err.kind, error::Kind::InvalidInput);
    assert_eq!(err.audience, error::Audience::User);
    assert!(err.message.starts_with("The AssurnaceProcedure could not be created: The procedure information has an issue: "));
}
#[test]
fn builder_error_invalid_procedure_description() {
    let result = AssuranceProcedure::builder()
        .api_version("1.0.0")
        .procedure_info("nrn:sourcecode::example", "A Short Desc.", "")
        .try_build();

    assert!(result.is_err(), "{}", format!("An was expected by one was not returned: {:?}", result.unwrap() ) );

    let err = result.unwrap_err();
    assert_eq!(err.kind, error::Kind::InvalidInput);
    assert_eq!(err.audience, error::Audience::User);
    assert!(err.message.starts_with("The AssurnaceProcedure could not be created: The procedure information has an issue: "));
}

/*** Assurnace Procedure Definition - Sad Path Tests ***/
/*** NOTE - There are no Procedure Definition Sad Paths in the current implementation ***/


/***Artifacts Sad Path Tests ***/
#[test]
fn builder_error_add_artifact_duplicate_artifact() {
    let result = AssuranceProcedure::builder()
        .api_version("1.0.0")
        .procedure_info("nrn:sourcecode::example", "A Short Desc.", "This is an example procedure")
        .add_activity(&Activity::new("procedure-1", "Short Desc", "Long Desc").unwrap())
        .add_artifact(&Artifact::new("artifact-1", "Short Desc", &vec![("key".to_string(), "value".to_string())]).unwrap())
        .add_artifact(&Artifact::new("artifact-1", "Short Desc", &vec![("key".to_string(), "value".to_string())]).unwrap())
        .try_build();

    assert!(result.is_err(), "{}", format!("An error was expected but one was not returned: {:?}", result.unwrap() ) );

    let err = result.unwrap_err();
    assert_eq!(err.kind, error::Kind::InvalidInput);
    assert_eq!(err.audience, Audience::User);
    assert!(err.message.starts_with("The AssurnaceProcedure could not be created: The artifact 'artifact-1' has an issue: "));
}
