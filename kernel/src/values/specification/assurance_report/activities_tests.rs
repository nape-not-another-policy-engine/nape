
use nape_testing_assertions::kernel_error_starts_with;
use crate::algorithms::signature_algorithm::{Signature};
use crate::algorithms::signature_algorithm::SignatureType::SHA256;
use crate::error::{Audience, Kind};
use crate::values::specification::assurance_report::action::Action;
use crate::values::specification::outcome::Outcome;
use crate::values::specification::assurance_report::activity::Activity;
use crate::values::specification::assurance_report::activities::Activities;
use crate::values::specification::assurance_report::signed_file::SignedFile;
use crate::values::specification::name::Name;


/** Positive Tests **/

#[test]
fn success() {
    let activities = Activities::builder().try_build().unwrap();
    assert_eq!(activities.list().len(), 0);
    assert_eq!(activities.action_count(), 0);
}


#[test]
fn add_single_activity_success() {

    let action1 = generate_testing_action("action-1");
    let action2 = generate_testing_action("action-2");

    let activity = Activity::builder().name("some-activity")
        .add(&action1).add(&action2).try_build().unwrap();

    let activities = Activities::builder().add_activity(&activity).try_build().unwrap();

    assert_eq!(activities.list().len(), 1);
    assert_eq!(activities.list()[0], activity);
    assert_eq!(activities.list()[0].name, Name::try_from("some-activity").unwrap());
    assert_eq!(activities.action_count(), 2);
}

#[test]
fn add_activity_with_existing_activity_name_success() {

    let action1 = generate_testing_action("action-1");
    let action2 = generate_testing_action("action-2");
    let action3 = generate_testing_action("action-3");
    let action4 = generate_testing_action("action-4");

    let activity1 = Activity::builder().name("activity-1").add(&action1).add(&action2).try_build().unwrap();
    let activity2 = Activity::builder().name("activity-1").add(&action3).add(&action4).try_build().unwrap();
    let activities = Activities::builder().add_activity(&activity1).add_activity(&activity2).try_build().unwrap();

    assert_eq!(activities.list().len(), 1);
    assert_eq!(activities.action_count(), 4);

}


#[test]
fn add_activity_with_same_name_actions_error() {

    let action1 = generate_testing_action("action-1");
    let action2 = generate_testing_action("action-2");
    let action3 = generate_testing_action("action-3");

    let activity1 = Activity::builder().name("activity-1")
        .add(&action1).add(&action2).try_build().unwrap();
    let activity2 = Activity::builder().name("activity-1")
        .add(&action3).add(&action1).try_build().unwrap();

    let result = Activities::builder().add_activity(&activity1).add_activity(&activity2).try_build();

    kernel_error_starts_with!(&result, Kind::InvalidInput, Audience::User, "The Activities list could not be created. ");
}

#[test]
fn add_action_to_empty_activities_success() {


    let test_file_1_sig = Signature::try_new(SHA256, "the-test-file-1-signature").unwrap();
    let test_file_1 = SignedFile::new("./some-location/file-1.txt", &test_file_1_sig).unwrap();
    let evidence_file_1_sig = Signature::try_new(SHA256, "the-evidence-file-1-signature").unwrap();
    let evidence_file_1 = SignedFile::new("./some-location/file-1.txt", &evidence_file_1_sig).unwrap();

    let action_1 = Action::builder()
        .name("action-1")
        .use_outcome(&Outcome::PASS)
        .reason("action reason")
        .use_test_file_signature(&test_file_1)
        .use_evidence_file_signature(&evidence_file_1)
        .try_build().unwrap();

    let activities = Activities::builder().add_action("activity-1", &action_1).try_build().unwrap();

    assert_eq!(activities.list().len(), 1);
    assert_eq!(activities.list()[0].name, Name::try_from("activity-1").unwrap());
    assert_eq!(activities.action_count(), 1);
    assert_eq!(activities.list()[0].actions[0], action_1);

}

#[test]
fn add_action_to_existing_activities_success() {

    let test_file_1_sig = Signature::try_new(SHA256, "the-test-file-1-signature").unwrap();
    let test_file_1 = SignedFile::new("./some-location/file-1.txt", &test_file_1_sig).unwrap();
    let evidence_file_1_sig = Signature::try_new(SHA256, "the-evidence-file-1-signature").unwrap();
    let evidence_file_1 = SignedFile::new("./some-location/file-1.txt", &evidence_file_1_sig).unwrap();

    let action_1 = Action::builder()
        .name("action-1")
        .use_outcome(&Outcome::PASS)
        .reason("action reason")
        .use_test_file_signature(&test_file_1)
        .use_evidence_file_signature(&evidence_file_1)
        .try_build().unwrap();

    let activity1 = Activity::builder().name("activity-1").try_build().unwrap();

    let activities =  Activities::builder()
        .add_activity(&activity1)
        .add_action("activity-1", &action_1)
        .try_build().unwrap();

    // Check that the activity was added to the existing activity
    assert_eq!(activities.list().len(), 1);
    assert_eq!(activities.list()[0].name, Name::try_from("activity-1").unwrap());
    assert_eq!(activities.action_count(), 1);
    assert_eq!(activities.list()[0].actions[0], action_1);
}

#[test]
fn add_action_with_new_activity_success() {

    let test_file_1_sig = Signature::try_new(SHA256, "the-test-file-1-signature").unwrap();
    let test_file_1 = SignedFile::new("./some-location/file-1.txt", &test_file_1_sig).unwrap();
    let evidence_file_1_sig = Signature::try_new(SHA256, "the-evidence-file-1-signature").unwrap();
    let evidence_file_1 = SignedFile::new("./some-location/file-1.txt", &evidence_file_1_sig).unwrap();

    let action_1 = Action::builder()
        .name("action-1")
        .use_outcome(&Outcome::PASS)
        .reason("action reason")
        .use_test_file_signature(&test_file_1)
        .use_evidence_file_signature(&evidence_file_1)
        .try_build().unwrap();

    let activity1 = Activity::builder().name("activity-1").try_build().unwrap();

    let activities = Activities::builder()
        .add_activity(&activity1)
        .add_action("activity-2", &action_1)
        .try_build().unwrap();

    // Check that the new procuedre exists with the activity
    assert_eq!(activities.list().len(), 2);
    assert_eq!(activities.list()[1].name, Name::try_from("activity-2").unwrap());
    assert_eq!(activities.action_count(), 1);
    assert_eq!(activities.list()[1].actions[0], action_1);
}

/** Negative Tests **/

#[test]
fn add_action_error_invalid_activity_name() {

    let test_file_1_sig = Signature::try_new(SHA256, "the-test-file-1-signature").unwrap();
    let test_file_1 = SignedFile::new("./some-location/file-1.txt", &test_file_1_sig).unwrap();
    let evidence_file_1_sig = Signature::try_new(SHA256, "the-evidence-file-1-signature").unwrap();
    let evidence_file_1 = SignedFile::new("./some-location/file-1.txt", &evidence_file_1_sig).unwrap();

    let action_1 = Action::builder()
        .name("action-1")
        .use_outcome(&Outcome::PASS)
        .reason("action reason")
        .use_test_file_signature(&test_file_1)
        .use_evidence_file_signature(&evidence_file_1)
        .try_build().unwrap();

    let result = Activities::builder().add_action("invalid activity name", &action_1).try_build();

    kernel_error_starts_with!(&result, Kind::InvalidInput, Audience::User, "The Activities list could not be created. ");

}


fn generate_testing_action(name: &str) -> Action {  Action::builder()
        .name(name)
        .outcome("pass")
        .reason("action reason")
        .test_file_path("./some-test/file.txt")
        .test_file_signature("SHA256[testsignature]")
        .evidence_file_path("./some-evidence/file.txt")
        .evidence_file_signature("SHA256[evidencesignature]")
        .try_build().unwrap()
}