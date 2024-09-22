use nape_kernel::values::specification::assurance_report::action::Action;
use nape_kernel::values::specification::assurance_report::activity::Activity;
use nape_kernel::values::specification::v1_0_0::assurance_report::AssuranceReportV1;
use crate::gateway_adapter::serde::specification_serializer::assurance_report::v1_0_0::AssuranceReportFileV1;


#[test]
fn success() {

    let action1 = Action::builder().name("action-1").outcome("pass").reason("Test passed").test_file_path("test_file.txt").test_file_signature("SHA256[theaction1testsig]").evidence_file_path("evidence_file.txt").evidence_file_signature("SHA256[theaction1evidencesig]").try_build().unwrap();
    let action2 = Action::builder().name("action-2").outcome("fail").reason("Test failed").test_file_path("test_file.txt").test_file_signature("SHA256[theaction2testsig]").evidence_file_path("evidence_file.txt").evidence_file_signature("SHA256[theaction2evidencesig]").try_build().unwrap();
    let action3 = Action::builder().name("action-3").outcome("inconclusive").reason("Test inconclusive").test_file_path("test_file.txt").test_file_signature("SHA256[theaction3testsig]").evidence_file_path("evidence_file.txt").evidence_file_signature("SHA256[theaction3evidencesig]").try_build().unwrap();
    let action4 = Action::builder().name("action-4").outcome("pass").reason("Test passed").test_file_path("test_file.txt").test_file_signature("SHA256[theaction4testsig]").evidence_file_path("evidence_file.txt").evidence_file_signature("SHA256[theaction4evidencesig]").try_build().unwrap();

    let activity1 = Activity::builder().name("activity-1").add(&action1).add(&action2).try_build().unwrap();
    let activity2 = Activity::builder().name("activity-2").add(&action3).add(&action4).try_build().unwrap();

    // Assemble
    let report = AssuranceReportV1::builder()
        .subject_nrn("nrn:sourcecode:nape:nape-cli")
        .subject_id("9f3f183a300501b53e2fa04f48acb4bd478d6414")
        .add_metadata("build-id", "1")
        .procedure_repository("github.com/nape/processes")
        .procedure_directory("rust_ci/sourcecode_integration")
        .add_activity(&activity1).add_activity(&activity2)
        // tod - add additinoal info
        .try_build()
        .unwrap();

    // Act
    let report_file = AssuranceReportFileV1::from(&report);

    // Assert
    assert_eq!(report_file.api_version, "1.0.0");
    assert_eq!(report_file.kind, "AssuranceReport");
    assert_eq!(report_file.metadata.as_ref().unwrap().len(), 1);
    assert_eq!(report_file.metadata.as_ref().unwrap().get("build-id").unwrap(), "1");
    assert_eq!(report_file.subject.urn, "nrn:sourcecode:nape:nape-cli");
    assert_eq!(report_file.subject.id, "9f3f183a300501b53e2fa04f48acb4bd478d6414");
    assert_eq!(report_file.procedure.repository, "git://github.com/nape/processes");
    assert_eq!(report_file.procedure.directory, "rust_ci/sourcecode_integration");

    assert_eq!(report_file.summary.activity_count, 2);
    assert_eq!(report_file.summary.action_count, 4);
    assert_eq!(report_file.summary.actions_run, 4);
    assert_eq!(report_file.summary.pass, 2);
    assert_eq!(report_file.summary.fail, 1);
    assert_eq!(report_file.summary.inconclusive, 1);
    assert_eq!(report_file.summary.outcome, "inconclusive");

    let actual_activity1 = report_file.activities.get(0).unwrap();
    let actual_activity2 = report_file.activities.get(1).unwrap();
    assert_eq!(actual_activity1.name, "activity-1");
    assert_eq!(actual_activity2.name, "activity-2");

    let actual_action1 = actual_activity1.actions.get(0).unwrap();
    assert_eq!(actual_action1.name, "action-1");
    assert_eq!(actual_action1.outcome, "pass");
    assert_eq!(actual_action1.reason, "Test passed");
    assert_eq!(actual_action1.test_file.file, "test_file.txt");
    assert_eq!(actual_action1.test_file.signature, "SHA256[theaction1testsig]");
    assert_eq!(actual_action1.evidence_file.file, "evidence_file.txt");
    assert_eq!(actual_action1.evidence_file.signature, "SHA256[theaction1evidencesig]");

    let actual_action2 = actual_activity1.actions.get(1).unwrap();
    assert_eq!(actual_action2.name, "action-2");
    assert_eq!(actual_action2.outcome, "fail");
    assert_eq!(actual_action2.reason, "Test failed");
    assert_eq!(actual_action2.test_file.file, "test_file.txt");
    assert_eq!(actual_action2.test_file.signature, "SHA256[theaction2testsig]");
    assert_eq!(actual_action2.evidence_file.file, "evidence_file.txt");
    assert_eq!(actual_action2.evidence_file.signature, "SHA256[theaction2evidencesig]");

    let actual_action3 = actual_activity2.actions.get(0).unwrap();
    assert_eq!(actual_action3.name, "action-3");
    assert_eq!(actual_action3.outcome, "inconclusive");
    assert_eq!(actual_action3.reason, "Test inconclusive");
    assert_eq!(actual_action3.test_file.file, "test_file.txt");
    assert_eq!(actual_action3.test_file.signature, "SHA256[theaction3testsig]");
    assert_eq!(actual_action3.evidence_file.file, "evidence_file.txt");
    assert_eq!(actual_action3.evidence_file.signature, "SHA256[theaction3evidencesig]");

    let actual_action4 = actual_activity2.actions.get(1).unwrap();
    assert_eq!(actual_action4.name, "action-4");
    assert_eq!(actual_action4.outcome, "pass");
    assert_eq!(actual_action4.reason, "Test passed");
    assert_eq!(actual_action4.test_file.file, "test_file.txt");
    assert_eq!(actual_action4.test_file.signature, "SHA256[theaction4testsig]");
    assert_eq!(actual_action4.evidence_file.file, "evidence_file.txt");
    assert_eq!(actual_action4.evidence_file.signature, "SHA256[theaction4evidencesig]");
}

