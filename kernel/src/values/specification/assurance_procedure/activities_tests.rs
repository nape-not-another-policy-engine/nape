
use crate::error::{Audience, Kind};
use crate::values::specification::assurance_procedure::action::Action;
use crate::values::specification::assurance_procedure::activity::Activity;
use crate::values::specification::assurance_procedure::activities::Activities;

/** Happy path tests **/

#[test]
fn default_activities_success() {
    let activities = Activities::default();
    assert_eq!(activities.count(), 0);
    assert_eq!(activities.action_count(), 0);
}
#[test]
fn add_activity_success() {
    let activities = Activities { list: Vec::new() };
    let result = activities.add("activity-1", "Short Desc", "Long Desc");

    assert!(result.is_ok(), "{}", format!("An error was not expected: {:?}", result.err() ) );

    let new_activities = result.unwrap();
    assert_eq!(new_activities.count(), 1);
    assert_eq!(new_activities.list[0].name.value, "activity-1");
    assert_eq!(new_activities.action_count(), 0);

}
#[test]
fn merge_activity_success() {
    let activities = Activities { list: Vec::new() };
    let activity = Activity::new("activity-1", "Short Desc", "Long Desc").unwrap();
    let new_activities = activities.merge(&activity);
    assert_eq!(new_activities.count(), 1);
    assert_eq!(new_activities.list[0].name.value, "activity-1");
    assert_eq!(new_activities.action_count(), 0);
}
#[test]
fn add_activity_when_activity_exists_success() {
    let activities = Activities::default();

    let activity_one = Activity::new("activity-1", "Short Desc", "Long Desc").unwrap();
    let activities = activities.merge(&activity_one);

    let action = Action::builder().name("action-1")
        .short_description("Short Desc")
        .long_description("Long Desc")
        .test_file_path("test_file")
        .evidence_file_path("evidence")
        .try_build().unwrap();
    let activities = activities.add_activity("activity-1", &action).unwrap();

    assert_eq!(activities.count(), 1);
    assert_eq!(activities.action_count(), 1);
}
#[test]
fn merge_duplicate_activities_without_duplication_success() {
    let activities = Activities::default();

    let action_1 = Action::builder().name("action-1")
        .short_description("Short Desc")
        .long_description("Long Desc")
        .test_file_path("test_file")
        .evidence_file_path("evidence")
        .try_build().unwrap();
    let action_2 = Action::builder().name("action-2")
        .short_description("Short Desc")
        .long_description("Long Desc")
        .test_file_path("test_file")
        .evidence_file_path("evidence")
        .try_build().unwrap();

    let copy_action_1 = Action::builder().name("action-1")
        .short_description("Action 1 Short Desc")
        .long_description("Action 1 Long Desc")
        .test_file_path("test_file")
        .evidence_file_path("evidence")
        .try_build().unwrap();
    let copy_action_2 = Action::builder().name("action-2")
        .short_description("Action 2 Short Desc")
        .long_description("Action 2 Long Desc")
        .test_file_path("test_file")
        .evidence_file_path("evidence")
        .try_build().unwrap();

    let activity_one = Activity::new("action-1", "Short Desc", "Long Desc").unwrap()
        .add(action_1)
        .add(action_2);

    let activity_one_copy =  Activity::new("action-1", "Short Desc", "Long Desc").unwrap()
        .add(copy_action_1)
        .add(copy_action_2);

    let activities = activities
        .merge(&activity_one)
        .merge(&activity_one_copy);

    assert_eq!(activities.count(), 1);
    assert_eq!(activities.action_count(), 2);

}
#[test]
fn add_duplicate_activity_to_existing_activity_without_duplication_success() {
    let activities = Activities::default();

    let activity_one = Activity::new("activity-1", "Short Desc", "Long Desc").unwrap();
    let activities = activities.merge(&activity_one);

    let action_1 = Action::builder().name("action-1")
        .short_description("Short Desc")
        .long_description("Long Desc")
        .test_file_path("test_file")
        .evidence_file_path("evidence")
        .try_build().unwrap();
    let activities = activities.add_activity("activity-1", &action_1).unwrap();

    let copy_action_1 = Action::builder().name("action-1")
        .short_description("Short Desc")
        .long_description("Long Desc")
        .test_file_path("test_file")
        .evidence_file_path("evidence")
        .try_build().unwrap();

    let activities = activities.add_activity("activity-1", &copy_action_1).unwrap();

    assert_eq!(activities.count(), 1);
    assert_eq!(activities.action_count(), 1);
}


/** Sad path tests **/
#[test]
fn add_activity_error() {
    let activities = Activities::default();
    let result = activities.add("", "Short Desc", "Long Desc");
    assert!(result.is_err(), "{}", format!("An error was expected but one was not returned: {:?}", result.unwrap() ) );

    let err = result.unwrap_err();
    assert_eq!(err.kind, Kind::InvalidInput);
    assert_eq!(err.audience, Audience::User);
    assert!(err.message.starts_with("We could not add the activity '' to the list of activities: "));
}
#[test]
fn add_action_without_existing_activity_error() {
    let activities = Activities::default();

    let action_1 = Action::builder().name("action-1")
        .short_description("Short Desc")
        .long_description("Long Desc")
        .test_file_path("test_file")
        .evidence_file_path("evidence")
        .try_build().unwrap();
    let activities = activities.add_activity("activity-1", &action_1);

    assert!(activities.is_err(), "{}", format!("An error was expected but one was not returned: {:?}", activities.unwrap() ) );

    let err = activities.unwrap_err();
    assert_eq!(err.kind, Kind::InvalidInput);
    assert_eq!(err.audience, Audience::User);
    assert_eq!(err.message, "Activity 'activity-1' does not exist. The activity must exist before you can add an action to it.  Please add a activity with the name you provided, a short description, and a long description.");
}


