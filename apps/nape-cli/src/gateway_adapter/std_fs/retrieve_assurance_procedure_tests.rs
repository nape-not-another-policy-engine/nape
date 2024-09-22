use nape_kernel::values::specification;
use nape_testing_assertions::is_ok;
use nape_testing_filesystem::{canonical_path, create_file, remove};
use crate::gateway_adapter::std_fs::retrieve_assurance_procedure::from_yaml_on_filesystem;

#[test]
fn success() {

    // Clean up space if any previous test failed
    remove!("retrieve_assurance_procedure_success");

    // Assemble
    let file_content =  assurance_procedure_yaml();
    let file_path_buff = create_file!("retrieve_assurance_procedure_success/assurance_procedure.yaml",  &file_content );
    let canonical_path = canonical_path!(file_path_buff);

    // Act
    let result = from_yaml_on_filesystem(&canonical_path);

    // Assert
    is_ok!(&result);

    let assurance_procedure = result.unwrap();

    assert_eq!(assurance_procedure.api_version.as_string(), "1.0.0");
    assert_eq!(assurance_procedure.kind, specification::kind::Kind::AssuranceProcedure);
    assert_eq!(assurance_procedure.procedure.nrn.value, "nrn:procedure:nape/software-procedures:rust-ci/sourcecode-integration");
    assert_eq!(assurance_procedure.procedure.short.value, "Rust CI Assurance Procedure");
    assert_eq!(assurance_procedure.procedure.description.value, "This is the complete procedure required to integrate a source code change for a rust application\nback into the trunk, or main branch, of the source code repository.");
    assert_eq!(assurance_procedure.activities.list.len(), 1);
    assert_eq!(assurance_procedure.activities.list[0].name.value, "peer-review");
    assert_eq!(assurance_procedure.activities.list[0].short.value, "Peer Review");
    assert_eq!(assurance_procedure.activities.list[0].description.value, "This activity verifies that all actions require for a successfull peer review were completed.");
    assert_eq!(assurance_procedure.activities.list[0].actions.len(), 2);
    assert_eq!(assurance_procedure.activities.list[0].actions[0].name.value, "at-least-two-reviewers");
    assert_eq!(assurance_procedure.activities.list[0].actions[0].short.value, "Two (2) Reviewer Approval");
    assert_eq!(assurance_procedure.activities.list[0].actions[0].description.value, "There are at least two (2) peer reviews who approved the merge request.");
    assert_eq!(assurance_procedure.activities.list[0].actions[0].test.as_str(), "./activity/peer_review/at_least_two_reviewers.py");
    assert_eq!(assurance_procedure.activities.list[0].actions[0].evidence.as_str(), "./evidence/peer_review/peer_review.json");
    assert_eq!(assurance_procedure.activities.list[0].actions[1].name.value, "requester-not-a-reviewer");
    assert_eq!(assurance_procedure.activities.list[0].actions[1].short.value, "Requester is not Approver");
    assert_eq!(assurance_procedure.activities.list[0].actions[1].description.value, "The person who initiated the merge request is not one of the people who approved the merge request.");
    assert_eq!(assurance_procedure.activities.list[0].actions[1].test.as_str(), "./activity/peer_review/requester_not_a_reviewer.py");
    assert_eq!(assurance_procedure.activities.list[0].actions[1].evidence.as_str(), "./evidence/peer_review/peer_review.json");

    // Clean up
    remove!("retrieve_assurance_procedure_success");

}

// todo - test for "could not open file"
// todo - test for "could not read file"
// todo - test for "could not deserialize file content"
// todo - test for "failed to convert assurance procedure file content to AssuranceProcedure"

fn assurance_procedure_yaml() -> String {
    r#"---
apiVersion: 1.0.0
kind: AssuranceProcedure
procedure:
  nrn: "nrn:procedure:nape/software-procedures:rust-ci/sourcecode-integration"
  short: "Rust CI Assurance Procedure"
  description: |
    This is the complete procedure required to integrate a source code change for a rust application
    back into the trunk, or main branch, of the source code repository.
activity:
  - name: peer-review
    short: "Peer Review"
    description: "This activity verifies that all actions require for a successfull peer review were completed."
    expect-evidence:
      - "./evidence/peer_review/peer_review.json"
    action:
      - name: at-least-two-reviewers
        short: "Two (2) Reviewer Approval"
        description: "There are at least two (2) peer reviews who approved the merge request."
        test: "./activity/peer_review/at_least_two_reviewers.py"
        evidence: "./evidence/peer_review/peer_review.json"
      - name: requester-not-a-reviewer
        short: "Requester is not Approver"
        description: "The person who initiated the merge request is not one of the people who approved the merge request."
        test: "./activity/peer_review/requester_not_a_reviewer.py"
        evidence: "./evidence/peer_review/peer_review.json"
"#.to_string()

}