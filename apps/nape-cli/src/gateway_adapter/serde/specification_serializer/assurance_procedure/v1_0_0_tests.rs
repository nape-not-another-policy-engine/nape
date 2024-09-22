use nape_kernel::error::{Audience, Kind};
use nape_kernel::values::specification::assurance_procedure::action::Action;
use nape_kernel::values::specification::assurance_procedure::activity::Activity;
use nape_kernel::values::specification::v1_0_0::assurance_procedure::AssuranceProcedure;
use nape_testing_assertions::{is_ok, kernel_error_starts_with};
use crate::gateway_adapter::serde::specification_serializer::assurance_procedure::v1_0_0::{AssuranceProcedureFile, Procedure};
use crate::gateway_adapter::serde::specification_serializer::assurance_procedure::v1_0_0::Action as FileAction;
use crate::gateway_adapter::serde::specification_serializer::assurance_procedure::v1_0_0::Activity as FileActivity;

#[test]
fn from_success() {
    let procedure = generate_valid_assurance_procedure();
    let file = AssuranceProcedureFile::from(&procedure);

    assert_eq!(file.api_version, "1.0.0");
    assert_eq!(file.kind, "AssuranceProcedure");
    assert_eq!(file.procedure.nrn, "nrn:sourcecode::example");
    assert_eq!(file.procedure.short, "A Short Desc.");
    assert_eq!(file.procedure.description, "This is an example procedure");
    assert_eq!(file.activities.len(), 2);

    let activity_1 = file.activities.iter().find(|&activity| activity.name == "activity-1");
    assert!(activity_1.is_some(), "The activity-1 was not found in the file");
    let activity_1 = activity_1.unwrap();
    assert_eq!(activity_1.short, "Short Desc - A1");
    assert_eq!(activity_1.description, "Long Desc - Activity 1");
    assert_eq!(activity_1.actions.len(), 2);

    let action_1 = activity_1.actions.iter().find(|&action| action.name == "action-1");
    assert!(action_1.is_some(), "The action-1 was not found in the file");
    let action_1 = action_1.unwrap();
    assert_eq!(action_1.short, "Short Desc - A1");
    assert_eq!(action_1.description, "Long Desc - Action 1");
    assert_eq!(action_1.test, "test/for/action_1.txt");
    assert_eq!(action_1.evidence, "evidence/for/action_1.txt");

    let action_2 = activity_1.actions.iter().find(|&action| action.name == "action-2");
    assert!(action_2.is_some(), "The action-2 was not found in the file");
    let action_2 = action_2.unwrap();
    assert_eq!(action_2.short, "Short Desc - A2");
    assert_eq!(action_2.description, "Long Desc - Action 2");
    assert_eq!(action_2.test, "test/for/action_2.txt");
    assert_eq!(action_2.evidence, "evidence/for/action_2.txt");

    let activity_2 = file.activities.iter().find(|&activity| activity.name == "activity-2");
    assert!(activity_2.is_some(), "The activity-2 was not found in the file");
    let activity_2 = activity_2.unwrap();
    assert_eq!(activity_2.short, "Short Desc- A2");
    assert_eq!(activity_2.description, "Long Desc - Activity 2");
    assert_eq!(activity_2.actions.len(), 2);

    let action_3 = activity_2.actions.iter().find(|&action| action.name == "action-3");
    assert!(action_3.is_some(), "The action-3 was not found in the file");
    let action_3 = action_3.unwrap();
    assert_eq!(action_3.short, "Short Desc - A3");
    assert_eq!(action_3.description, "Long Desc - Action 3");
    assert_eq!(action_3.test, "test/for/action_3.txt");
    assert_eq!(action_3.evidence, "evidence/for/action_3.txt");

    let action_4 = activity_2.actions.iter().find(|&action| action.name == "action-4");
    assert!(action_4.is_some(), "The action-4 was not found in the file");
    let action_4 = action_4.unwrap();
    assert_eq!(action_4.short, "Short Desc - A4");
    assert_eq!(action_4.description, "Long Desc - Action 4");
    assert_eq!(action_4.test, "test/for/action_4.txt");
    assert_eq!(action_4.evidence, "evidence/for/action_4.txt");
}

#[test]
fn try_to_success() {
    let file = generate_valid_assurance_procedure_file();
    let result = file.try_to();

    is_ok!(&result);

    assert!(result.is_ok(), "The conversion failed when it was expected to succeed: {:?}", result);
    let result = result.unwrap();

    assert_eq!(result.api_version.as_string(), "1.0.0".to_string());
    assert_eq!(result.kind.to_string(), "AssuranceProcedure".to_string());
    assert_eq!(result.procedure.nrn.value, "nrn:sourcecode::example".to_string());
    assert_eq!(result.procedure.short.value, "A Short Desc.".to_string());
    assert_eq!(result.procedure.description.value, "This is an example procedure".to_string());
    assert_eq!(result.activities.list.len(), 2);

    let activity_1 = result.activities.list.iter().find(|&activity| activity.name.value == "activity-1");
    assert!(activity_1.is_some(), "The activity-1 was not found in the result");
    let activity_1 = activity_1.unwrap();
    assert_eq!(activity_1.short.value, "Short Desc - A1".to_string());
    assert_eq!(activity_1.description.value, "Long Desc - Activity 1".to_string());
    assert_eq!(activity_1.actions.len(), 2);

    let action_1 = activity_1.actions.iter().find(|&action| action.name.value == "action-1");
    assert!(action_1.is_some(), "The action-1 was not found in the result");
    let action_1 = action_1.unwrap();
    assert_eq!(action_1.short.value, "Short Desc - A1".to_string());
    assert_eq!(action_1.description.value, "Long Desc - Action 1".to_string());
    assert_eq!(action_1.test.as_str(), "test/for/action_1.txt".to_string());
    assert_eq!(action_1.evidence.as_str(), "evidence/for/action_1.txt".to_string());

    let action_2 = activity_1.actions.iter().find(|&action| action.name.value == "action-2");
    assert!(action_2.is_some(), "The action-2 was not found in the result");
    let action_2 = action_2.unwrap();
    assert_eq!(action_2.short.value, "Short Desc - A2".to_string());
    assert_eq!(action_2.description.value, "Long Desc - Action 2".to_string());
    assert_eq!(action_2.test.as_str(), "test/for/action_2.txt".to_string());
    assert_eq!(action_2.evidence.as_str(), "evidence/for/action_2.txt".to_string());

    let activity_2 = result.activities.list.iter().find(|&activity| activity.name.value == "activity-2");
    assert!(activity_2.is_some(), "The activity-2 was not found in the result");
    let activity_2 = activity_2.unwrap();
    assert_eq!(activity_2.short.value, "Short Desc- A2".to_string());
    assert_eq!(activity_2.description.value, "Long Desc - Activity 2".to_string());
    assert_eq!(activity_2.actions.len(), 2);

    let action_3 = activity_2.actions.iter().find(|&action| action.name.value == "action-3");
    assert!(action_3.is_some(), "The action-3 was not found in the result");
    let action_3 = action_3.unwrap();
    assert_eq!(action_3.short.value, "Short Desc - A3".to_string());
    assert_eq!(action_3.description.value, "Long Desc - Action 3".to_string());
    assert_eq!(action_3.test.as_str(), "test/for/action_3.txt".to_string());
    assert_eq!(action_3.evidence.as_str(), "evidence/for/action_3.txt".to_string());

    let action_4 = activity_2.actions.iter().find(|&action| action.name.value == "action-4");
    assert!(action_4.is_some(), "The action-4 was not found in the result");
    let action_4 = action_4.unwrap();
    assert_eq!(action_4.short.value, "Short Desc - A4".to_string());
    assert_eq!(action_4.description.value, "Long Desc - Action 4".to_string());
    assert_eq!(action_4.test.as_str(), "test/for/action_4.txt".to_string());
    assert_eq!(action_4.evidence.as_str(), "evidence/for/action_4.txt".to_string());
}

#[test]
fn try_to_handles_invalid_api_version_error() {
    let file = generate_invalid_assurance_procedure_file_bad_api_version();
    let result = file.try_to();

    kernel_error_starts_with!(result, Kind::ProcessingFailure, Audience::System, "Failed to extract the data from the Assurance Procedure File. " );
}

#[test]
fn try_to_handles_invalid_procedure_error() {
    let file = generate_invalid_assurance_procedure_file_bad_procedure();
    let result = file.try_to();

    kernel_error_starts_with!(result, Kind::ProcessingFailure, Audience::System, "Failed to extract the data from the Assurance Procedure File. " );
}

#[test]
fn try_to_handles_invalid_activity_error() {
    let file = generate_invalid_assurance_procedure_file_activity();
    let result = file.try_to();

    kernel_error_starts_with!(result, Kind::ProcessingFailure, Audience::System, "Failed to extract the data from the Assurance Procedure File. There is an issue with an Activity. " );
}

#[test]
fn try_to_handles_invalid_action_error() {
    let file = generate_invalid_assurance_procedure_file_bad_action();
    let result = file.try_to();

    kernel_error_starts_with!(result, Kind::ProcessingFailure, Audience::System, "Failed to extract the data from the Assurance Procedure File. There is an issue with an Action. " );
}




fn generate_valid_assurance_procedure() -> AssuranceProcedure {

    let activity_1 = Activity::new("activity-1", "Short Desc - A1", "Long Desc - Activity 1").unwrap();
    let activity_1 = activity_1.add(Action::builder().name("action-1")
        .short_description("Short Desc - A1")
        .long_description("Long Desc - Action 1")
        .test_file_path("test/for/action_1.txt")
        .evidence_file_path("evidence/for/action_1.txt")
        .try_build().unwrap()
    );
    let activity_1 = activity_1.add(Action::builder().name("action-2")
        .short_description("Short Desc - A2")
        .long_description("Long Desc - Action 2")
        .test_file_path("test/for/action_2.txt")
        .evidence_file_path("evidence/for/action_2.txt")
        .try_build().unwrap()
    );


    let activity_2 = Activity::new("activity-2", "Short Desc- A2", "Long Desc - Activity 2").unwrap();
    let activity_2 = activity_2.add(Action::builder().name("action-3")
        .short_description("Short Desc - A3")
        .long_description("Long Desc - Action 3")
        .test_file_path("test/for/action_3.txt")
        .evidence_file_path("evidence/for/action_3.txt")
        .try_build().unwrap()
    );
    let activity_2 = activity_2.add(Action::builder().name("action-4")
        .short_description("Short Desc - A4")
        .long_description("Long Desc - Action 4")
        .test_file_path("test/for/action_4.txt")
        .evidence_file_path("evidence/for/action_4.txt")
        .try_build().unwrap()
    );


    AssuranceProcedure::builder()
        .api_version("1.0.0")
        .procedure_info("nrn:sourcecode::example", "A Short Desc.", "This is an example procedure")
        .add_activity(&activity_1)
        .add_activity(&activity_2)
        .try_build().unwrap()
}

fn generate_valid_assurance_procedure_file() -> AssuranceProcedureFile {

    let action1 = FileAction {  name: "action-1".to_string(),  short: "Short Desc - A1".to_string(),  description: "Long Desc - Action 1".to_string(),  test: "test/for/action_1.txt".to_string(),  evidence: "evidence/for/action_1.txt".to_string() };
    let action2 = FileAction {  name: "action-2".to_string(),  short: "Short Desc - A2".to_string(),  description: "Long Desc - Action 2".to_string(),  test: "test/for/action_2.txt".to_string(),  evidence: "evidence/for/action_2.txt".to_string() };
    let action3 = FileAction {  name: "action-3".to_string(),  short: "Short Desc - A3".to_string(),  description: "Long Desc - Action 3".to_string(),  test: "test/for/action_3.txt".to_string(),  evidence: "evidence/for/action_3.txt".to_string() };
    let action4 = FileAction {  name: "action-4".to_string(),  short: "Short Desc - A4".to_string(),  description: "Long Desc - Action 4".to_string(),  test: "test/for/action_4.txt".to_string(),  evidence: "evidence/for/action_4.txt".to_string() };

    let activity1 = FileActivity {  name: "activity-1".to_string(),  short: "Short Desc - A1".to_string(),  description: "Long Desc - Activity 1".to_string(),  actions: vec![action1, action2] };
    let activity2 = FileActivity {  name: "activity-2".to_string(),  short: "Short Desc- A2".to_string(),  description: "Long Desc - Activity 2".to_string(),  actions: vec![action3, action4] };

    let procedure = Procedure { nrn: "nrn:sourcecode::example".to_string(),  short: "A Short Desc.".to_string(),  description: "This is an example procedure".to_string() };

    AssuranceProcedureFile {  api_version: "1.0.0".to_string(),  kind: "AssuranceProcedure".to_string(),  procedure,  activities: vec![activity1, activity2] }

}


fn generate_invalid_assurance_procedure_file_bad_api_version() -> AssuranceProcedureFile {

    // The action name that is bad is action1, everything else is ok.
    let action1 = FileAction {  name: "action-1".to_string(),  short: "Short Desc - A1".to_string(),  description: "Long Desc - Action 1".to_string(),  test: "test/for/action_1.txt".to_string(),  evidence: "evidence/for/action_1.txt".to_string() };
    let action2 = FileAction {  name: "action-2".to_string(),  short: "Short Desc - A2".to_string(),  description: "Long Desc - Action 2".to_string(),  test: "test/for/action_2.txt".to_string(),  evidence: "evidence/for/action_2.txt".to_string() };
    let action3 = FileAction {  name: "action-3".to_string(),  short: "Short Desc - A3".to_string(),  description: "Long Desc - Action 3".to_string(),  test: "test/for/action_3.txt".to_string(),  evidence: "evidence/for/action_3.txt".to_string() };
    let action4 = FileAction {  name: "action-4".to_string(),  short: "Short Desc - A4".to_string(),  description: "Long Desc - Action 4".to_string(),  test: "test/for/action_4.txt".to_string(),  evidence: "evidence/for/action_4.txt".to_string() };

    let activity1 = FileActivity {  name: "activity-1".to_string(),  short: "Short Desc - A1".to_string(),  description: "Long Desc - Activity 1".to_string(),  actions: vec![action1, action2] };
    let activity2 = FileActivity {  name: "activity-2".to_string(),  short: "Short Desc- A2".to_string(),  description: "Long Desc - Activity 2".to_string(),  actions: vec![action3, action4] };

    let procedure = Procedure { nrn: "nrn:sourcecode::example".to_string(),  short: "A Short Desc.".to_string(),  description: "This is an example procedure".to_string() };

    AssuranceProcedureFile {  api_version: "bad version".to_string(),  kind: "AssuranceProcedure".to_string(),  procedure,  activities: vec![activity1, activity2] }

}


fn generate_invalid_assurance_procedure_file_bad_procedure() -> AssuranceProcedureFile {

    let action1 = FileAction {  name: "action-1".to_string(),  short: "Short Desc - A1".to_string(),  description: "Long Desc - Action 1".to_string(),  test: "test/for/action_1.txt".to_string(),  evidence: "evidence/for/action_1.txt".to_string() };
    let action2 = FileAction {  name: "action-2".to_string(),  short: "Short Desc - A2".to_string(),  description: "Long Desc - Action 2".to_string(),  test: "test/for/action_2.txt".to_string(),  evidence: "evidence/for/action_2.txt".to_string() };
    let action3 = FileAction {  name: "action-3".to_string(),  short: "Short Desc - A3".to_string(),  description: "Long Desc - Action 3".to_string(),  test: "test/for/action_3.txt".to_string(),  evidence: "evidence/for/action_3.txt".to_string() };
    let action4 = FileAction {  name: "action-4".to_string(),  short: "Short Desc - A4".to_string(),  description: "Long Desc - Action 4".to_string(),  test: "test/for/action_4.txt".to_string(),  evidence: "evidence/for/action_4.txt".to_string() };

    let activity1 = FileActivity {  name: "activity-1".to_string(),  short: "Short Desc - A1".to_string(),  description: "Long Desc - Activity 1".to_string(),  actions: vec![action1, action2] };
    let activity2 = FileActivity {  name: "activity-2".to_string(),  short: "Short Desc- A2".to_string(),  description: "Long Desc - Activity 2".to_string(),  actions: vec![action3, action4] };

    let procedure = Procedure { nrn: "a bad nrn".to_string(),  short: "A Short Desc.".to_string(),  description: "This is an example procedure".to_string() };

    AssuranceProcedureFile {  api_version: "1.0.0".to_string(),  kind: "AssuranceProcedure".to_string(),  procedure,  activities: vec![activity1, activity2] }

}

fn generate_invalid_assurance_procedure_file_activity() -> AssuranceProcedureFile {

    let action1 = FileAction {  name: "action-1".to_string(),  short: "Short Desc - A1".to_string(),  description: "Long Desc - Action 1".to_string(),  test: "test/for/action_1.txt".to_string(),  evidence: "evidence/for/action_1.txt".to_string() };
    let action2 = FileAction {  name: "action-2".to_string(),  short: "Short Desc - A2".to_string(),  description: "Long Desc - Action 2".to_string(),  test: "test/for/action_2.txt".to_string(),  evidence: "evidence/for/action_2.txt".to_string() };
    let action3 = FileAction {  name: "action-3".to_string(),  short: "Short Desc - A3".to_string(),  description: "Long Desc - Action 3".to_string(),  test: "test/for/action_3.txt".to_string(),  evidence: "evidence/for/action_3.txt".to_string() };
    let action4 = FileAction {  name: "action-4".to_string(),  short: "Short Desc - A4".to_string(),  description: "Long Desc - Action 4".to_string(),  test: "test/for/action_4.txt".to_string(),  evidence: "evidence/for/action_4.txt".to_string() };

    let activity1 = FileActivity {  name: "bad activity".to_string(),  short: "Short Desc - A1".to_string(),  description: "Long Desc - Activity 1".to_string(),  actions: vec![action1, action2] };
    let activity2 = FileActivity {  name: "activity-2".to_string(),  short: "Short Desc- A2".to_string(),  description: "Long Desc - Activity 2".to_string(),  actions: vec![action3, action4] };

    let procedure = Procedure { nrn: "nrn:sourcecode::example".to_string(),  short: "A Short Desc.".to_string(),  description: "This is an example procedure".to_string() };

    AssuranceProcedureFile {  api_version: "1.0.0".to_string(),  kind: "AssuranceProcedure".to_string(),  procedure,  activities: vec![activity1, activity2] }

}

fn generate_invalid_assurance_procedure_file_bad_action() -> AssuranceProcedureFile {

    let action1 = FileAction {  name: "bad action".to_string(),  short: "Short Desc - A1".to_string(),  description: "Long Desc - Action 1".to_string(),  test: "test/for/action_1.txt".to_string(),  evidence: "evidence/for/action_1.txt".to_string() };
    let action2 = FileAction {  name: "action-2".to_string(),  short: "Short Desc - A2".to_string(),  description: "Long Desc - Action 2".to_string(),  test: "test/for/action_2.txt".to_string(),  evidence: "evidence/for/action_2.txt".to_string() };
    let action3 = FileAction {  name: "action-3".to_string(),  short: "Short Desc - A3".to_string(),  description: "Long Desc - Action 3".to_string(),  test: "test/for/action_3.txt".to_string(),  evidence: "evidence/for/action_3.txt".to_string() };
    let action4 = FileAction {  name: "action-4".to_string(),  short: "Short Desc - A4".to_string(),  description: "Long Desc - Action 4".to_string(),  test: "test/for/action_4.txt".to_string(),  evidence: "evidence/for/action_4.txt".to_string() };

    let activity1 = FileActivity {  name: "activity-1".to_string(),  short: "Short Desc - A1".to_string(),  description: "Long Desc - Activity 1".to_string(),  actions: vec![action1, action2] };
    let activity2 = FileActivity {  name: "activity-2".to_string(),  short: "Short Desc- A2".to_string(),  description: "Long Desc - Activity 2".to_string(),  actions: vec![action3, action4] };

    let procedure = Procedure { nrn: "nrn:sourcecode::example".to_string(),  short: "A Short Desc.".to_string(),  description: "This is an example procedure".to_string() };

    AssuranceProcedureFile {  api_version: "1.0.0".to_string(),  kind: "AssuranceProcedure".to_string(),  procedure,  activities: vec![activity1, activity2] }

}