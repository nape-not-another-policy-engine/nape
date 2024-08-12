
use crate::error::{Audience, Kind};
use crate::values::specification::assurance_procedure::action::Action;
use crate::values::specification::assurance_procedure::activity::Activity;

/** Happy path tests **/

#[test]
fn new_success() {
    let result = Activity::new("procedure-1", "Short Desc", "Long Desc");

    assert!(result.is_ok(), "{}", format!("An error was expected but one was not returned: {:?}", result.unwrap() ) );

    let procedure = result.unwrap();
    assert_eq!(procedure.name.value, "procedure-1");
    assert_eq!(procedure.short.value, "Short Desc");
    assert_eq!(procedure.description.value, "Long Desc");
    assert_eq!(procedure.expected_evidence.len(), 0);
    assert_eq!(procedure.action_count(), 0);
}

#[test]
fn add_activity_success() {
    let activity = Activity::new("procedure-1", "Short Desc", "Long Desc").unwrap();
    let action = Action::builder().name("action-1")
        .short_description("Short Desc")
        .long_description("Long Desc")
        .test_file_path("test_file_path")
        .evidence_file_path("evidence_file_path")
        .try_build().unwrap();

    let updated_activity = activity.add(action);

    assert_eq!(updated_activity.action_count(), 1);
    assert_eq!(updated_activity.actions[0].name.value, "action-1");
    assert_eq!(updated_activity.actions[0].short.value, "Short Desc");
    assert_eq!(updated_activity.actions[0].description.value, "Long Desc");
    assert_eq!(updated_activity.actions[0].test.as_str(), "test_file_path");
    assert_eq!(updated_activity.actions[0].evidence.as_str(), "evidence_file_path");

}

#[test]
fn activity_add_expected_evidence_success() {
    let  activity = Activity::new("activity-1", "Short Desc", "Long Desc").unwrap();
    let result = activity.add_expected_evidence("file_path");
    assert!(result.is_ok(), "{}", format!("An error was expected but one was not returned: {:?}", result.unwrap() ) );

    let activity = result.unwrap();
    assert_eq!(activity.expected_evidence.len(), 1);
    assert_eq!(activity.expected_evidence[0].as_str(), "file_path");
    assert_eq!(activity.action_count(), 0);

}

/** Sad path tests **/

#[test]
fn new_activity_error_invalid_name() {
    let result = Activity::new("", "Short Desc", "Long Desc");
    assert!(result.is_err(), "{}", format!("An error was expected but one was not returned: {:?}", result.unwrap() ) );

    let err = result.unwrap_err();
    assert_eq!(err.kind, Kind::InvalidInput);
    assert_eq!(err.audience, Audience::User);
    assert!(err.message.starts_with("There is an issue with the activity information: The name has an issue: "));
}

#[test]
fn new_activity_error_invalid_short_desc() {
    let result = Activity::new("activity-1", "", "Long Desc");
    assert!(result.is_err(), "{}", format!("An error was expected but one was not returned: {:?}", result.unwrap() ) );

    let err = result.unwrap_err();
    assert_eq!(err.kind, Kind::InvalidInput);
    assert_eq!(err.audience, Audience::User);
    assert!(err.message.starts_with("There is an issue with the activity information: The short description has an issue: "));
}

#[test]
fn new_activity_error_long_desc() {
    let result = Activity::new("activity-1", "Short Desc", "");
    assert!(result.is_err(), "{}", format!("An error was expected but one was not returned: {:?}", result.unwrap() ) );

    let err = result.unwrap_err();
    assert_eq!(err.kind, Kind::InvalidInput);
    assert_eq!(err.audience, Audience::User);
    assert!(err.message.starts_with("There is an issue with the activity information: The long description has an issue: "));
}

#[test]
fn activity_add_expected_evidence_error() {
    let  activity = Activity::new("activity-1", "Short Desc", "Long Desc").unwrap();
    let result = activity.add_expected_evidence("");
    assert!(result.is_err(), "{}", format!("An error was expected but one was not returned: {:?}", result.unwrap() ) );

    let err = result.unwrap_err();
    assert_eq!(err.kind, Kind::InvalidInput);
    assert_eq!(err.audience, Audience::User);
    assert!(err.message.starts_with("There is an issue with the activity information: There was an issue adding the expected evidence '': "));
}