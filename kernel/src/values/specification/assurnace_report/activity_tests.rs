use nape_testing_assertions::kernel_error_starts_with;
use crate::algorithms::signature_algorithm::Signature;
use crate::algorithms::signature_algorithm::SignatureType::SHA256;
use crate::error::{Audience, Kind};
use crate::values::specification::assurnace_report::action::Action;
use crate::values::specification::outcome::Outcome;
use crate::values::specification::assurnace_report::activity::Activity;
use crate::values::specification::assurnace_report::signed_file::SignedFile;
use crate::values::specification::name::Name;

#[test]
fn builder_success() {
    let activity = Activity::builder().name("activity-name").try_build().unwrap();
    assert_eq!(activity.name(), &Name::try_from("activity-name").unwrap());
    assert_eq!(activity.count(), 0);
}

#[test]
fn builder_use_override_success() {
    let activity = Activity::builder()
        .use_name(&Name::try_from("override-name").unwrap())
        .name("activity-name")
        .try_build().unwrap();
    assert_eq!(activity.name(), &Name::try_from("override-name").unwrap());
    assert_eq!(activity.count(), 0);
}

#[test]
fn add_action_success() {
    let test_file_sig = Signature::try_new(SHA256, "the-test-file-signature").unwrap();
    let test_file = SignedFile::new("./some-location/file.txt", &test_file_sig).unwrap();
    let evidence_file_sig = Signature::try_new(SHA256, "the-evidence-file-signature").unwrap();
    let evidence_file = SignedFile::new("./some-location/file.txt", &evidence_file_sig).unwrap();

    let action = Action::builder().name("action-name")
        .use_outcome(&Outcome::PASS)
        .reason("action reason")
        .use_test_file_signature(&test_file)
        .use_evidence_file_signature(&evidence_file)
        .try_build().unwrap();

    let activity = Activity::builder()
        .name("activity-name")
        .add(&action)
        .try_build().unwrap();

    assert_eq!(activity.actions(), &vec![action]);
    assert_eq!(activity.count(), 1);
}

#[test]
fn add_many_actions_success() {
    let test_file_sig = Signature::try_new(SHA256, "the-test-file-signature").unwrap();
    let test_file = SignedFile::new("./some-location/file.txt", &test_file_sig).unwrap();
    let evidence_file_sig = Signature::try_new(SHA256, "the-evidence-file-signature").unwrap();
    let evidence_file = SignedFile::new("./some-location/file.txt", &evidence_file_sig).unwrap();

    let action_1 = Action::builder().name("action-1").use_outcome(&Outcome::PASS).reason("action reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action_2 = Action::builder().name("action-2").use_outcome(&Outcome::PASS).reason("action reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action_3 = Action::builder().name("action-3").use_outcome(&Outcome::PASS).reason("action reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action_4 = Action::builder().name("action-4").use_outcome(&Outcome::PASS).reason("action reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action_5 = Action::builder().name("action-5").use_outcome(&Outcome::PASS).reason("action reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action_6 = Action::builder().name("action-6").use_outcome(&Outcome::PASS).reason("action reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action_7 = Action::builder().name("action-7").use_outcome(&Outcome::PASS).reason("action reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action_8 = Action::builder().name("action-8").use_outcome(&Outcome::PASS).reason("action reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action_9 = Action::builder().name("action-9").use_outcome(&Outcome::PASS).reason("action reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action_10 = Action::builder().name("action-10").use_outcome(&Outcome::PASS).reason("action reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();

    let activity = Activity::builder()
        .name("activity-name")
        .add(&action_1)
        .add(&action_2)
        .add(&action_3)
        .add(&action_4)
        .add(&action_5)
        .add(&action_6)
        .add(&action_7)
        .add(&action_8)
        .add(&action_9)
        .add(&action_10)
        .try_build().unwrap();

    assert_eq!(activity.actions(), &vec![action_1, action_2, action_3, action_4, action_5, action_6, action_7, action_8, action_9, action_10]);
    assert_eq!(activity.count(), 10);
}

#[test]
fn invalid_no_name_error() {

    let result = Activity::builder().try_build();

    kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "Please provide an Activity name.");

}

#[test]
fn invalid_name_error() {

    let result = Activity::builder().name("invalid name").try_build();

    kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User,"The activity name 'invalid name' is invalid. ");

}

#[test]
fn duplicate_action_error() {

    let action = Action::builder().name("action-name").outcome("pass").reason("reason").test_file_path("test_file.txt").test_file_signature("SHA256[thesig]").evidence_file_path("evidence_file.txt").evidence_file_signature("SHA256[thesig]").try_build().unwrap();
    let dup_action = Action::builder().name("action-name").outcome("pass").reason("reason duplicative").test_file_path("test_file.txt").test_file_signature("SHA256[thesig]").evidence_file_path("evidence_file.txt").evidence_file_signature("SHA256[thesig]").try_build().unwrap();

    let result = Activity::builder().name("activity-name").add(&action).add(&dup_action).try_build();

    kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "The activity 'activity-name' has one or more actions with the same action name of 'action-name'. All action names for a given activity must be unique. Review the actions for this activity to ensure each action has a unique name.");

}



