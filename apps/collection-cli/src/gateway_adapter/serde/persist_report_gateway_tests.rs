use crate::gateway_adapter::serde::persist_report_gateway::save_report_as_yaml;
use nape_kernel::values::specification::v1_0_0::assurance_report::AssuranceReportV1;
use nape_testing_assertions::is_ok;
use nape_testing_filesystem::{canonical_path, create, remove};
use nape_kernel::values::specification::assurance_report::action::Action;
use nape_kernel::values::specification::assurance_report::activity::Activity;

// Each Assurance Report version has it's won module as a way to keep the code organized.

/// Verify [`AssuranceReportV1`] can be persisted as a YAML file, and is persisted successfully.
mod persist_assurance_report_v1 {
    use std::fs;
    use super::*;

    // TODO - Negative path tests - Directory provided does not exist
    // TODO - Negative path tests - Directory provided is not a directory
    // TODO - Negative path tests - Directory provided is not writable
    // TODO - Negative path tests - Person cannpt access the metadata of the directory

    #[test]
    fn success() {

        // Assemble
        let report_directory_path = create!("persist_report_gateway_success");
        let report_dir_string = canonical_path!(&report_directory_path);

        let _action1 = Action::builder().name("action-1").outcome("pass").reason("Test passed").test_file_path("test_file.txt").test_file_signature("SHA256[thesig]").evidence_file_path("evidence_file.txt").evidence_file_signature("SHA256[thesig]").try_build().unwrap();
        let _action2 = Action::builder().name("action-2").outcome("fail").reason("Test failed").test_file_path("test_file.txt").test_file_signature("SHA256[thesig]").evidence_file_path("evidence_file.txt").evidence_file_signature("SHA256[thesig]").try_build().unwrap();
        let _action3 = Action::builder().name("action-3").outcome("inconclusive").reason("Test inconclusive").test_file_path("test_file.txt").test_file_signature("SHA256[thesig]").evidence_file_path("evidence_file.txt").evidence_file_signature("SHA256[thesig]").try_build().unwrap();
        let _action4 = Action::builder().name("action-4").outcome("pass").reason("Test passed").test_file_path("test_file.txt").test_file_signature("SHA256[thesig]").evidence_file_path("evidence_file.txt").evidence_file_signature("SHA256[thesig]").try_build().unwrap();

        let action1 = Action::builder().name("action-1").outcome("pass").reason("Test passed").test_file_path("test_file.txt").test_file_signature("SHA256[theaction1testsig]").evidence_file_path("evidence_file.txt").evidence_file_signature("SHA256[theaction1evidencesig]").try_build().unwrap();
        let action2 = Action::builder().name("action-2").outcome("fail").reason("Test failed").test_file_path("test_file.txt").test_file_signature("SHA256[theaction2testsig]").evidence_file_path("evidence_file.txt").evidence_file_signature("SHA256[theaction2evidencesig]").try_build().unwrap();
        let action3 = Action::builder().name("action-3").outcome("inconclusive").reason("Test inconclusive").test_file_path("test_file.txt").test_file_signature("SHA256[theaction3testsig]").evidence_file_path("evidence_file.txt").evidence_file_signature("SHA256[theaction3evidencesig]").try_build().unwrap();
        let action4 = Action::builder().name("action-4").outcome("pass").reason("Test passed").test_file_path("test_file.txt").test_file_signature("SHA256[theaction4testsig]").evidence_file_path("evidence_file.txt").evidence_file_signature("SHA256[theaction4evidencesig]").try_build().unwrap();
        let action5 = Action::builder().name("action-5").outcome("pass").reason("Test passed").test_file_path("test_file.txt").test_file_signature("SHA256[theaction5testsig]").evidence_file_path("evidence_file.txt").evidence_file_signature("SHA256[theaction5evidencesig]").try_build().unwrap();
        let action6 = Action::builder().name("action-6").outcome("fail").reason("Test failed").test_file_path("test_file.txt").test_file_signature("SHA256[theaction6testsig]").evidence_file_path("evidence_file.txt").evidence_file_signature("SHA256[theaction6evidencesig]").try_build().unwrap();
        let action7 = Action::builder().name("action-7").outcome("inconclusive").reason("Test inconclusive").test_file_path("test_file.txt").test_file_signature("SHA256[theaction7testsig]").evidence_file_path("evidence_file.txt").evidence_file_signature("SHA256[theaction7evidencesig]").try_build().unwrap();

        let activity1 = Activity::builder().name("activity-1").add(&action1).add(&action2).try_build().unwrap();
        let activity2 = Activity::builder().name("activity-2").add(&action3).add(&action4).try_build().unwrap();
        let activity3 = Activity::builder().name("activity-3").add(&action5).add(&action6).add(&action7).try_build().unwrap();

        let report = AssuranceReportV1::builder()
            .subject_nrn("nrn:sourcecode:nape:collection-cli")
            .subject_id("9f3f183a300501b53e2fa04f48acb4bd478d6414")
            .add_metadata("build-id", "1")
            .procedure_repository("github.com/nape/processes.git")
            .procedure_directory("rust_ci/sourcecode_integration")
            .add_activity(&activity1)
            .add_activity(&activity2)
            .add_activity(&activity3)
            .try_build()
            .unwrap();

        // Act
        let result = save_report_as_yaml(&report, &report_dir_string);


        // Assert
        is_ok!(&result);

        // Read the contents of the file
        let result_file_path = result.unwrap();
        let result_contents = fs::read_to_string(result_file_path.as_str()).expect("Unable to read file");

        // Compare with expected YAML
        let expected_contents = expected_yaml();
        assert_eq!(result_contents, expected_contents, "The YAML contents do not match");

        // Cleanup
        remove!("persist_report_gateway_success")
    }


    fn expected_yaml() -> String {
        r#"apiVersion: 1.0.0
kind: AssuranceReport
metadata:
  build-id: '1'
subject:
  urn: nrn:sourcecode:nape:collection-cli
  id: 9f3f183a300501b53e2fa04f48acb4bd478d6414
procedure:
  repository: https://github.com/nape/processes.git
  directory: rust_ci/sourcecode_integration
summary:
  activity_count: 3
  action_count: 7
  actions_run: 7
  pass: 3
  fail: 2
  inconclusive: 2
  outcome: inconclusive
activity:
- name: activity-1
  action:
  - name: action-1
    outcome: pass
    reason: Test passed
    test_file:
      file: test_file.txt
      signature: SHA256[theaction1testsig]
    evidence_file:
      file: evidence_file.txt
      signature: SHA256[theaction1evidencesig]
  - name: action-2
    outcome: fail
    reason: Test failed
    test_file:
      file: test_file.txt
      signature: SHA256[theaction2testsig]
    evidence_file:
      file: evidence_file.txt
      signature: SHA256[theaction2evidencesig]
- name: activity-2
  action:
  - name: action-3
    outcome: inconclusive
    reason: Test inconclusive
    test_file:
      file: test_file.txt
      signature: SHA256[theaction3testsig]
    evidence_file:
      file: evidence_file.txt
      signature: SHA256[theaction3evidencesig]
  - name: action-4
    outcome: pass
    reason: Test passed
    test_file:
      file: test_file.txt
      signature: SHA256[theaction4testsig]
    evidence_file:
      file: evidence_file.txt
      signature: SHA256[theaction4evidencesig]
- name: activity-3
  action:
  - name: action-5
    outcome: pass
    reason: Test passed
    test_file:
      file: test_file.txt
      signature: SHA256[theaction5testsig]
    evidence_file:
      file: evidence_file.txt
      signature: SHA256[theaction5evidencesig]
  - name: action-6
    outcome: fail
    reason: Test failed
    test_file:
      file: test_file.txt
      signature: SHA256[theaction6testsig]
    evidence_file:
      file: evidence_file.txt
      signature: SHA256[theaction6evidencesig]
  - name: action-7
    outcome: inconclusive
    reason: Test inconclusive
    test_file:
      file: test_file.txt
      signature: SHA256[theaction7testsig]
    evidence_file:
      file: evidence_file.txt
      signature: SHA256[theaction7evidencesig]
"#.to_string()
    }

}
