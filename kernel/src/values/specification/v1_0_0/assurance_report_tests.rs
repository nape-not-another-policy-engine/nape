use nape_testing_assertions::is_ok;
use nape_testing_assertions::kernel_error_starts_with;
use crate::algorithms::signature_algorithm::Signature;
use crate::algorithms::signature_algorithm::SignatureType::SHA256;
use crate::error::{Audience, Kind};
use crate::values;
use crate::values::specification::v1_0_0::assurance_report::Builder;
use crate::values::specification::assurance_report::action::Action;
use crate::values::specification::assurance_report::activities::Activities;
use crate::values::specification::outcome::Outcome;
use crate::values::specification::assurance_report::activity::Activity;
use crate::values::specification::assurance_report::additional_information::AdditionalInformation;
use crate::values::specification::assurance_report::signed_file::SignedFile;
use crate::values::specification::description::Description;
use crate::values::specification::metadata::MetaData;
use crate::values::specification::name::Name;
use crate::values::specification::procedure::Procedure;
use crate::values::specification::subject::Subject;
use crate::values::specification::traits::AssuranceReport;

#[test]
fn builder_success()  {
    let existing_metadata = vec![("key-1".to_string(), "value 1".to_string()), ("key-2".to_string(), "value 2".to_string())];

    let result  = Builder::new()
        .add_metadata("key", "value")
        .merge_metadata(&existing_metadata)
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid" )
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_activity(&Activity::builder().name("activity").try_build().unwrap())
        .add_action("procedure", &testing_acttion() )
        .additional_information("additional information")
        .try_build();

    assert!(result.is_ok(), "An error was not expected, but was returned: {:?}", result.err().unwrap());

    let report = result.unwrap();
    assert_eq!(report.api_version().as_string(), "1.0.0".to_string());
    assert_eq!(report.kind(), values::specification::kind::Kind::AssuranceReport);
    assert_eq!(report.metadata().data.len(), 3);
    assert_eq!(report.metadata().data[0].0, Name::try_from("key").unwrap());
    assert_eq!(report.metadata().data[0].1, Description::try_from("value").unwrap());
    assert_eq!(report.subject().nrn.value, "nrn:procedure:example");
    assert_eq!(report.subject().id.value, "somesubjectid");
    assert_eq!(report.procedure().repository, "https://some-location.com");
    assert_eq!(report.activities().action_count(), 1);
    assert_eq!(report.additional_info().count(), 1);
    assert_eq!(report.additional_info().list()[0].value, "additional information");
    assert_eq!(report.summary().action_count, 1);
    assert_eq!(report.summary().actions_run, 1);
    assert_eq!(report.summary().pass, 1);
    assert_eq!(report.summary().fail, 0);
    assert_eq!(report.summary().inconclusive, 0);
    assert_eq!(report.summary().outcome, Outcome::PASS);

}

#[test]
fn builder_multiple_metadata_activities_actions_and_additional_information_success() {
    let test_file_sig = Signature::try_new(SHA256, "the-test-file-signature").unwrap();
    let test_file = SignedFile::new("./some-location/file.txt", &test_file_sig).unwrap();
    let evidence_file_sig = Signature::try_new(SHA256, "the-evidence-file-signature").unwrap();
    let evidence_file = SignedFile::new("./some-location/file.txt", &evidence_file_sig).unwrap();

    let action1 = Action::builder().name("action-name-1").use_outcome(&Outcome::PASS).reason("action reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action2 = Action::builder().name("action-name-2").use_outcome(&Outcome::PASS).reason("action reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();
    let action3 = Action::builder().name("action-name-3").use_outcome(&Outcome::PASS).reason("action reason").use_test_file_signature(&test_file).use_evidence_file_signature(&evidence_file).try_build().unwrap();

    let result = Builder::new()
        .add_metadata("key", "value")
        .add_metadata("key2", "value2")
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_activity(&Activity::builder().name("activity").try_build().unwrap())
        .add_activity(&Activity::builder().name("activity-1").try_build().unwrap())
        .add_activity(&Activity::builder().name("activity-2").try_build().unwrap())
        .add_action("activity", &action1)
        .add_action("activity-3", &action2)
        .add_action("activity-4", &action3)
        .additional_information("additional information")
        .additional_information("additional information 2")
        .try_build();

    assert!(result.is_ok(), "An error was not expected, but was returned: {:?}", result.err().unwrap());

    let report = result.unwrap();
    assert_eq!(report.api_version().as_string(), "1.0.0".to_string());
    assert_eq!(report.kind(), values::specification::kind::Kind::AssuranceReport);
    assert_eq!(report.metadata().data.len(), 2);
    assert_eq!(report.metadata().data[0].0, Name::try_from("key").unwrap());
    assert_eq!(report.metadata().data[0].1, Description::try_from("value").unwrap());
    assert_eq!(report.metadata().data[1].0, Name::try_from("key2").unwrap());
    assert_eq!(report.metadata().data[1].1, Description::try_from("value2").unwrap());
    assert_eq!(report.subject().nrn.value, "nrn:procedure:example");
    assert_eq!(report.subject().id.value, "somesubjectid");
    assert_eq!(report.procedure().repository, "https://some-location.com");
    assert_eq!(report.activities().list().len(), 5);
    assert_eq!(report.activities().list()[0].name.value, "activity");
    assert_eq!(report.activities().list()[1].name.value, "activity-1");
    assert_eq!(report.activities().list()[2].name.value, "activity-2");
    assert_eq!(report.activities().list()[3].name.value, "activity-3");
    assert_eq!(report.activities().list()[4].name.value, "activity-4");
    assert_eq!(report.additional_info().count(), 2);
    assert_eq!(report.additional_info().list()[0].value, "additional information");
    assert_eq!(report.additional_info().list()[1].value, "additional information 2");
    assert_eq!(report.summary().action_count, 3);
    assert_eq!(report.summary().actions_run, 3);
    assert_eq!(report.summary().pass, 3);
    assert_eq!(report.summary().fail, 0);
    assert_eq!(report.summary().inconclusive, 0);
    assert_eq!(report.summary().outcome, Outcome::PASS);
}

/// Verifies that when the *use_* functions are called, their values override the values set by their resepctive functions
///
/// For example if, [`use_metadata`] has been invoked, then any and all data added via the [`add_metadata`], [`merge_metadata`], or ['upsert_metadata'] functions will be ignored.
///
#[test]
fn use_functions_override_success() {
    let existing_metadata = vec![("key-1".to_string(), "value 1".to_string())];

    let mut overriding_metadata = MetaData::default();
    overriding_metadata.add("override-key", "value").unwrap();
    overriding_metadata.add("override-key-2", "value 2").unwrap();
    overriding_metadata.add("override-key-3", "value 3").unwrap();
    overriding_metadata.add("override-key-4", "value 4").unwrap();

    let overriding_subject = Subject::try_new("nrn:sourcecode:override/subject", "overridesubjectid").unwrap();
    let overriding_procedure = Procedure::try_new("https://some-override-location.com", "some/override/location").unwrap();

    let test_file_sig = SignedFile::new("./some-location/file.txt", &Signature::try_new(SHA256, "the-signature").unwrap()).unwrap();
    let evidence_file_sig = SignedFile::new("./some-location/file.txt", &Signature::try_new(SHA256, "the-signature").unwrap()).unwrap();

    let action1 = Action::builder().name("action-name-1").use_outcome(&Outcome::PASS).reason("action reason").use_test_file_signature(&test_file_sig).use_evidence_file_signature(&evidence_file_sig).try_build().unwrap();
    let action2 = Action::builder().name("action-name-2").use_outcome(&Outcome::PASS).reason("action reason").use_test_file_signature(&test_file_sig).use_evidence_file_signature(&evidence_file_sig).try_build().unwrap();
    let action3 = Action::builder().name("action-name-3").use_outcome(&Outcome::PASS).reason("action reason").use_test_file_signature(&test_file_sig).use_evidence_file_signature(&evidence_file_sig).try_build().unwrap();
    let action4 = Action::builder().name("action-name-4").use_outcome(&Outcome::PASS).reason("action reason").use_test_file_signature(&test_file_sig).use_evidence_file_signature(&evidence_file_sig).try_build().unwrap();

    let activity1 = Activity::builder().name("activity")
        .add(&action1).add(&action2).try_build().unwrap();

    let activity2 = Activity::builder().name("activity-2")
        .add(&action3).add(&action4).try_build().unwrap();

    let overriding_activities = Activities::builder().add_activity(&activity1).add_activity(&activity2).try_build().unwrap();

    let overriding_additional_info = AdditionalInformation::builder()
        .add("override-additional-info").add("override-additional-info-2").try_build().unwrap();


    let result  = Builder::new()
        .add_metadata("key", "value")
        .merge_metadata(&existing_metadata)
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid" )
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_activity( &Activity::builder().name("activity-first").try_build().unwrap() )
        .add_action("activity-first", &testing_acttion() )
        .additional_information("additional information first")
        .use_metadata(&overriding_metadata)
        .use_subject(&overriding_subject)
        .use_procedure(&overriding_procedure)
        .use_activities(&overriding_activities)
        .use_additional_information(&overriding_additional_info)

        .try_build();

    assert!(result.is_ok(), "An error was not expected, but was returned: {:?}", result.err().unwrap());

    let report = result.unwrap();
    assert_eq!(report.api_version().as_string(), "1.0.0".to_string());
    assert_eq!(report.kind(), values::specification::kind::Kind::AssuranceReport);
    assert_eq!(report.metadata().data.len(), 4);
    assert_eq!(report.subject().nrn.value, "nrn:sourcecode:override/subject");
    assert_eq!(report.subject().id.value, "overridesubjectid");
    assert_eq!(report.procedure().repository, "https://some-override-location.com");
    assert_eq!(report.procedure().directory, "some/override/location");
    assert_eq!(report.activities().action_count(), 4);
    assert_eq!(report.additional_info().count(), 2);
    assert_eq!(report.summary().action_count, 4);
    assert_eq!(report.summary().actions_run, 4);
    assert_eq!(report.summary().pass, 4);
    assert_eq!(report.summary().fail, 0);
    assert_eq!(report.summary().inconclusive, 0);
    assert_eq!(report.summary().outcome, Outcome::PASS);

}

#[test]
fn builder_no_action_or_activity_call_success() {
    let builder = Builder::new();

    let result = builder
        .add_metadata("key", "value")
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .additional_information("additional information")
        .try_build();

    is_ok !(&result);
    let result = result.unwrap();
    assert_eq!(result.activities().list().len(), 0);

}
#[test]
fn builder_no_activity_call_success() {
    let builder = Builder::new();

    let result = builder
        .add_metadata("key", "value")
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_action("procedure", &testing_acttion())
        .additional_information("additional information")
        .try_build();

    is_ok!(&result);
    let result = result.unwrap();
    assert_eq!(result.activities().list().len(), 1);

}
#[test]
fn builder_no_action_call_success() {
    let builder = Builder::new();

    let result = builder
        .add_metadata("key", "value")
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_activity(&Activity::builder().name("activity").try_build().unwrap())
        .additional_information("additional information")
        .try_build();

    is_ok!(&result);

    let result = result.unwrap();
    assert_eq!(result.activities().list().len(), 1);
}
#[test]
fn builder_no_additional_information_success() {
    let builder = Builder::new();

    let result = builder
        .add_metadata("key", "value")
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_activity(&Activity::builder().name("activity").try_build().unwrap())
        .add_action("procedure", &testing_acttion())
        .try_build();

    is_ok!(&result);
    let result = result.unwrap();
    assert_eq!(result.additional_info().count(), 0);
}

#[test]
fn builder_upsert_metadata_success() {
    // Initialize an AssuranceReport instance with some initial metadata
    let result  = Builder::new()
        .add_metadata("key", "value")
        .add_metadata("key-2", "value 2")
        .upsert_metadata("key", "updated_value")
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid" )
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_activity(&Activity::builder().name("activity").try_build().unwrap())
        .add_action("procedure", &testing_acttion() )
        .additional_information("additional information")
        .try_build();

    is_ok!(&result);
    let report = result.unwrap();

    assert_eq!(report.metadata().data.len(), 2);
    assert_eq!(report.metadata().get("key"), Some("updated_value".to_string()));
    assert_eq!(report.metadata().get("key-2"), Some("value 2".to_string()));

}

#[test]
fn builder_no_metadata_success() {
    let builder = Builder::new();

    let result = builder
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_activity(&Activity::builder().name("activity").try_build().unwrap())
        .add_action("procedure", &testing_acttion())
        .additional_information("additional information")
        .try_build();

    is_ok!(&result);

    let result = result.unwrap();
    assert_eq!(result.metadata().data.len(), 0 )
}
#[test]
fn builder_bad_metadata_error() {
    let builder = Builder::new();

    let report_result = builder
        .add_metadata("a bad key", "value")
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_activity(&Activity::builder().name("activity").try_build().unwrap())
        .add_action("procedure", &testing_acttion())
        .try_build();

    kernel_error_starts_with!(report_result, Kind::InvalidInput, Audience::User, "The AssuranceReport could not be created. The Metadata has an issue.");
}

#[test]
fn builder_no_subject_nrn_error() {
    let builder = Builder::new();

    let report_result = builder
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_metadata("key", "value")
        .procedure_repository("https://some-location.com")
        .add_activity(&Activity::builder().name("activity").try_build().unwrap())
        .add_action("procedure", &testing_acttion())
        .additional_information("additional information")
        .try_build();

    kernel_error_starts_with!(report_result, Kind::InvalidInput, Audience::User, "The AssuranceReport could not be created. The subject NRN is required, although it was not provided.");

}
#[test]
fn builder_bad_subject_nrn_error() {
    let builder = Builder::new();

    let report_result = builder
        .add_metadata("key", "value")
        .subject_nrn("some bad subject nrn")
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_activity(&Activity::builder().name("activity").try_build().unwrap())
        .add_action("procedure", &testing_acttion())
        .try_build();

    kernel_error_starts_with!(report_result, Kind::InvalidInput, Audience::User, "The AssuranceReport could not be created. There is an issue with your Subject data. ");
}

#[test]
fn builder_no_subject_id_error() {
    let builder = Builder::new();

    let report_result = builder
        .subject_nrn("nrn:procedure:example")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_metadata("key", "value")
        .procedure_repository("https://some-location.com")
        .add_activity(&Activity::builder().name("activity").try_build().unwrap())
        .add_action("procedure", &testing_acttion())
        .additional_information("additional information")
        .try_build();

    kernel_error_starts_with!(report_result, Kind::InvalidInput, Audience::User, "The AssuranceReport could not be created. The subject ID is required, although it was not provided.");

}
#[test]
fn builder_bad_subject_id_error() {
    let builder = Builder::new();

    let report_result = builder
        .add_metadata("key", "value")
        .subject_nrn("nrn:procedure:example")
        .subject_id("")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_activity(&Activity::builder().name("activity").try_build().unwrap())
        .add_action("procedure", &testing_acttion())
        .try_build();

    kernel_error_starts_with!(report_result, Kind::InvalidInput, Audience::User, "The AssuranceReport could not be created. There is an issue with your Subject data. ");
}

#[test]
fn builder_no_procedure_repository_error() {
    let builder = Builder::new();

    let report_result = builder
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_directory("some/location")
        .add_metadata("key", "value")
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .add_activity(&Activity::builder().name("activity").try_build().unwrap())
        .add_action("procedure", &testing_acttion())
        .additional_information("additional information")
        .try_build();

    kernel_error_starts_with!(report_result, Kind::InvalidInput, Audience::User, "The AssuranceReport could not be created. The procedure repository link is required, but was not provided.");
}
#[test]
fn builder_bad_procedure_repository_error() {
    let builder = Builder::new();

    let report_result = builder
        .add_metadata("key", "value")
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("bad procedure repo")
        .procedure_directory("some/location")
        .add_activity(&Activity::builder().name("activity").try_build().unwrap())
        .add_action("procedure", &testing_acttion())
        .try_build();

    kernel_error_starts_with!(report_result, Kind::InvalidInput, Audience::User, "The AssuranceReport could not be created. There is an issue with your procedure data. ");
}

#[test]
fn builder_no_procedure_directory_error() {
    let builder = Builder::new();

    let report_result = builder
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .add_metadata("key", "value")
        .add_activity(&Activity::builder().name("activity").try_build().unwrap())
        .add_action("procedure", &testing_acttion())
        .additional_information("additional information")
        .try_build();

    kernel_error_starts_with!(report_result, Kind::InvalidInput, Audience::User, "The AssuranceReport could not be created. The procedure directory is required, but was not provided.");
}
#[test]
fn builder_bad_procedure_directory_error() {
    let builder = Builder::new();

    let report_result = builder
        .add_metadata("key", "value")
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("https://example.com")
        .procedure_directory("/some/bad/location-/")
        .add_activity(&Activity::builder().name("activity").try_build().unwrap())
        .add_action("procedure", &testing_acttion())
        .try_build();

    kernel_error_starts_with!(report_result, Kind::InvalidInput, Audience::User, "The AssuranceReport could not be created. There is an issue with your procedure data. ");
}


#[test]
fn builder_bad_activity_name_error() {
    let builder = Builder::new();

    let report_result = builder
        .add_metadata("key", "value")
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_activity(&Activity::builder().name("good-procedure-name").try_build().unwrap())
        .add_action("bad- activity name", &testing_acttion())
        .try_build();

    kernel_error_starts_with!(report_result, Kind::InvalidInput, Audience::User, "The AssuranceReport could not be created. There is an issue adding your Activity. ");
}


#[test]
fn builder_additional_information_empty_string_error() {
    let builder = Builder::new();

    let report_result = builder
        .add_metadata("key", "value")
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_activity(&Activity::builder().name("activity").try_build().unwrap())
        .add_action("action", &testing_acttion())
        .additional_information("")
        .try_build();

    kernel_error_starts_with!(report_result, Kind::InvalidInput, Audience::User, "The AssuranceReport could not be created. There is an issue adding your Additional Information to the report. ");
}

/** Testing Helpers */

fn testing_acttion() -> Action {
    let test_file_sig = Signature::try_new(SHA256, "the-test-file-signature").unwrap();
    let test_file = SignedFile::new("./some-location/file.txt", &test_file_sig).unwrap();
    let evidence_file_sig = Signature::try_new(SHA256, "the-evidence-file-signature").unwrap();
    let evidence_file = SignedFile::new("./some-location/file.txt", &evidence_file_sig).unwrap();

    Action::builder()
        .name("action-name")
        .use_outcome(&Outcome::PASS)
        .reason("action reason")
        .use_test_file_signature(&test_file)
        .use_evidence_file_signature(&evidence_file)
        .try_build().unwrap()
}





