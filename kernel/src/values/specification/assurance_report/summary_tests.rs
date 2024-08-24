use crate::algorithms::signature_algorithm::Signature;
use crate::algorithms::signature_algorithm::SignatureType::SHA256;
use crate::values::specification::assurance_report::action::Action;
use crate::values::specification::outcome::Outcome;
use crate::values::specification::assurance_report::activity::Activity;
use crate::values::specification::assurance_report::activities::Activities;
use crate::values::specification::assurance_report::signed_file::SignedFile;
use crate::values::specification::assurance_report::summary::Summary;


#[test]
fn default_summary() {
    let summary = Summary::default();
    assert_eq!(summary.activity_count, 0);
    assert_eq!(summary.action_count, 0);
    assert_eq!(summary.actions_run, 0);
    assert_eq!(summary.pass, 0);
    assert_eq!(summary.fail, 0);
    assert_eq!(summary.inconclusive, 0);
    assert_eq!(summary.outcome, Outcome::INCONCLUSIVE);
}

#[test]
fn summary_of_success_outcome_pass() {
    let (test_file, evidence_file) = signed_file_helper();

    let action1 = Action::builder().name("Test-Action-1").use_outcome(&Outcome::PASS).reason("some reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action2 = Action::builder().name("Test-Action-2").use_outcome(&Outcome::PASS).reason("some reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action3 = Action::builder().name("Test-Action-3").use_outcome(&Outcome::PASS).reason("some reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action4 = Action::builder().name("Test-Action-4").use_outcome(&Outcome::PASS).reason("some reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();

    let activity = Activity::builder().name("Test-Activity")
        .add(&action1)
        .add(&action2)
        .add(&action3)
        .add(&action4)
        .try_build().unwrap();

    let activities = Activities::builder().add_activity(&activity).try_build().unwrap();

    let summary = Summary::of(&activities);

    assert_eq!(summary.activity_count, 1);
    assert_eq!(summary.action_count, 4);
    assert_eq!(summary.actions_run, 4);
    assert_eq!(summary.pass, 4);
    assert_eq!(summary.fail, 0);
    assert_eq!(summary.inconclusive, 0);
    assert_eq!(summary.outcome, Outcome::PASS);
}

#[test]
fn summary_of_success_outcome_inconclusive_with_at_least_one_fail() {
    let (test_file, evidence_file) = signed_file_helper();


    let action1 = Action::builder().name("Test-Action-1").use_outcome(&Outcome::PASS).reason("some reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action2 = Action::builder().name("Test-Action-2").use_outcome(&Outcome::PASS).reason("some reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action3 = Action::builder().name("Test-Action-3").use_outcome(&Outcome::PASS).reason("some reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action4 = Action::builder().name("Test-Action-4").use_outcome(&Outcome::FAIL).reason("some reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();

    let activity = Activity::builder().name("Test-Activity")
        .add(&action1)
        .add(&action2)
        .add(&action3)
        .add(&action4)
        .try_build().unwrap();

    let activities = Activities::builder().add_activity(&activity).try_build().unwrap();

    let summary = Summary::of(&activities);

    assert_eq!(summary.activity_count, 1);
    assert_eq!(summary.action_count, 4);
    assert_eq!(summary.actions_run, 4);
    assert_eq!(summary.pass, 3);
    assert_eq!(summary.fail, 1);
    assert_eq!(summary.inconclusive, 0);
    assert_eq!(summary.outcome, Outcome::FAIL);
}

#[test]
fn summary_of_success_outcome_inconclusive_with_at_least_one_inconclusive() {
    let (test_file, evidence_file) = signed_file_helper();

    let action1 = Action::builder().name("Test-Action-1").use_outcome(&Outcome::PASS).reason("some reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action2 = Action::builder().name("Test-Action-2").use_outcome(&Outcome::PASS).reason("some reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action3 = Action::builder().name("Test-Action-3").use_outcome(&Outcome::FAIL).reason("some reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action4 = Action::builder().name("Test-Action-4").use_outcome(&Outcome::INCONCLUSIVE).reason("some reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();

    let activity = Activity::builder().name("Test-Activity")
        .add(&action1)
        .add(&action2)
        .add(&action3)
        .add(&action4)
        .try_build().unwrap();

    let activities = Activities::builder().add_activity(&activity).try_build().unwrap();

    let summary = Summary::of(&activities);

    assert_eq!(summary.activity_count, 1);
    assert_eq!(summary.action_count, 4);
    assert_eq!(summary.actions_run, 4);
    assert_eq!(summary.pass, 2);
    assert_eq!(summary.fail, 1);
    assert_eq!(summary.inconclusive, 1);
    assert_eq!(summary.outcome, Outcome::INCONCLUSIVE);
}

#[test]
fn summary_of_success_outcome_inconclusive_with_at_least_one_error() {

    let (test_file, evidence_file) = signed_file_helper();
    let action1 = Action::builder().name("Test-Action-1").use_outcome(&Outcome::PASS).reason("some reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action2 = Action::builder().name("Test-Action-2").use_outcome(&Outcome::FAIL).reason("some reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action3 = Action::builder().name("Test-Action-3").use_outcome(&Outcome::INCONCLUSIVE).reason("some reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action4 = Action::builder().name("Test-Action-4").use_outcome(&Outcome::ERROR).reason("some reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();

    let activity = Activity::builder().name("Test-Activity-1")
        .add(&action1)
        .add(&action2)
        .add(&action3)
        .add(&action4)
        .try_build().unwrap();

    let activities = Activities::builder().add_activity(&activity).try_build().unwrap();

    let summary = Summary::of(&activities);

    assert_eq!(summary.activity_count, 1);
    assert_eq!(summary.action_count, 4);
    assert_eq!(summary.actions_run, 3);
    assert_eq!(summary.pass, 1);
    assert_eq!(summary.fail, 1);
    assert_eq!(summary.inconclusive, 1);
    assert_eq!(summary.outcome, Outcome::INCONCLUSIVE);
}


fn signed_file_helper() -> (SignedFile, SignedFile) {
    let test_file_sig = Signature::try_new(SHA256, "the-test-file-signature").unwrap();
    let test_file = SignedFile::new("./some-location/file.txt", &test_file_sig).unwrap();
    let evidence_file_sig = Signature::try_new(SHA256, "the-evidence-file-signature").unwrap();
    let evidence_file = SignedFile::new("./some-location/file.txt", &evidence_file_sig).unwrap();

    (test_file, evidence_file)

}