use nape_kernel::algorithms::signature_algorithm::{Signature, SignatureType};
use nape_kernel::error::Error;
use nape_kernel::values::specification::file_path::FilePath;
use nape_kernel::values::specification::kind;
use nape_kernel::values::specification::assurance_procedure::action::Action;
use nape_kernel::values::specification::assurance_procedure::activity::Activity;
use nape_testing_assertions::is_ok;
use crate::evidence_collection::usecases::evaluate_evidence::gateway_boundary::response::{EvaluationResults, TestResult};
use crate::evidence_collection::usecases::evaluate_evidence::usecase::AssuranceReportBuilder;
use crate::evidence_collection::usecases::evaluate_evidence::usecase_boundary::request::EvaluateEvidence;
use nape_kernel::error::{Audience, Kind};
use nape_kernel::values::specification::outcome::Outcome;
use nape_kernel::values::specification::v1_0_0::assurance_procedure::AssuranceProcedure;
use nape_kernel::values::specification::api_version::APIVersion;
use nape_testing_assertions::{kernel_error_eq, kernel_error_contains, kernel_error_starts_with};
use crate::evidence_collection::usecases::evaluate_evidence::gateway_boundary::request::EvaluationFiles;
use crate::evidence_collection::usecases::evaluate_evidence::usecase::evaluate_and_report;

mod usecase {
    use nape_kernel::values::specification::traits::AssuranceReport;
    use super::*;

    #[test]
    fn success() {
        let request = generate_valid_request();

        let report_result = evaluate_and_report(
            &request,
            mock_retrieve_directory_path,
            mock_retrieve_procedure_definition,
            mock_evaluate_evidence,
            mock_sig_algo,
            mock_file_data_gw,
            mock_persist_report_gw);

        is_ok!(&report_result);

    }

    #[test]
    fn no_retrieve_procedure_definition_doc_path_error() {
        let request = generate_valid_request();

        let report_result = evaluate_and_report(
            &request,
            mock_retrieve_directory_path_procedure_definition_doc_error,
            mock_retrieve_procedure_definition,
            mock_evaluate_evidence,
            mock_sig_algo,
            mock_file_data_gw,
            mock_persist_report_gw);

        // Make sure the error starts with the proper message
        kernel_error_starts_with!(&report_result, Kind::GatewayError, Audience::System,
            "Failed to retrieve the 'assurance-procedure-file' path. ");

        // Make sure the error contains the gateway error message.  There is a possiblity that there could be more text than what the error message starts with and the gateway error message, therefore that's why two assertions are used because these are the two pieces of context we want to ensure are there.
        kernel_error_contains!(&report_result, Kind::GatewayError, Audience::System,
            "Procedure Doc Error");

    }

    #[test]
    fn no_retrieve_home_path_error() {
        let request = generate_valid_request();

        let report_result = evaluate_and_report(
            &request,
            mock_retrieve_directory_path_home_error,
            mock_retrieve_procedure_definition,
            mock_evaluate_evidence,
            mock_sig_algo,
            mock_file_data_gw,
            mock_persist_report_gw);

        // Make sure the error starts with the proper message
        kernel_error_starts_with!(&report_result, Kind::GatewayError, Audience::System,
            "Failed to retrieve the 'home' directory path. ");

        // Make sure the error contains the gateway error message.  There is a possiblity that there could be more text than what the error message starts with and the gateway error message, therefore that's why two assertions are used because these are the two pieces of context we want to ensure are there.
        kernel_error_contains!(&report_result, Kind::GatewayError, Audience::System,
            "Home Path Error");

    }

    #[test]
    fn could_not_get_procedure_def_error() {
        let request = generate_valid_request();

        let report_result = evaluate_and_report(
            &request,
            mock_retrieve_directory_path,
            mock_retrieve_procedure_definition_error,
            mock_evaluate_evidence,
            mock_sig_algo,
            mock_file_data_gw,
            mock_persist_report_gw);

        // Make sure the error starts with the proper message
        kernel_error_starts_with!(&report_result, Kind::GatewayError, Audience::System,
            "Failed to retrieve procedure definition. ");

        // Make sure the error contains the gateway error message.  There is a possiblity that there could be more text than what the error message starts with and the gateway error message, therefore that's why two assertions are used because these are the two pieces of context we want to ensure are there.
        kernel_error_contains!(&report_result, Kind::GatewayError, Audience::System,
            "Could not get procedure definition");
    }

    #[test]
    fn evaluate_evidence_gateway_error() {
        let request = generate_valid_request();

        let report_result = evaluate_and_report(
            &request,
            mock_retrieve_directory_path,
            mock_retrieve_procedure_definition,
            mock_evaluate_evidence_error,
            mock_sig_algo,
            mock_file_data_gw,
            mock_persist_report_gw);

        // Make sure the error starts with the proper message
        kernel_error_starts_with!(&report_result, Kind::GatewayError, Audience::System,
            "Failed to evaluate evidence files. ");

        // Make sure the error contains the gateway error message.  There is a possibility that there could be more text than what the error message starts with and the gateway error message, therefore that's why two assertions are used because these are the two pieces of context we want to ensure are there.
        kernel_error_contains!(&report_result, Kind::GatewayError, Audience::System,
            "Could not evaluate evidence files");
    }

    #[test]
    fn sig_algo_error() {
        let request = generate_valid_request();

        let report_result = evaluate_and_report(
            &request,
            mock_retrieve_directory_path,
            mock_retrieve_procedure_definition,
            mock_evaluate_evidence,
            mock_sig_algo_error,
            mock_file_data_gw,
            mock_persist_report_gw);

        // Make sure the error starts with the proper message
        kernel_error_starts_with!(&report_result, Kind::ProcessingFailure, Audience::System,
            "Failed to generate assurance report. Failed to sign the file: ");

        // Make sure the error contains the gateway error message.  There is a possibility that there could be more text than what the error message starts with and the gateway error message, therefore that's why two assertions are used because these are the two pieces of context we want to ensure are there.
        kernel_error_contains!(&report_result, Kind::ProcessingFailure, Audience::System,
            "Signature Algorithm Error");
    }

    #[test]
    fn file_data_gateway_error() {
        let request = generate_valid_request();

        let report_result = evaluate_and_report(
            &request,
            mock_retrieve_directory_path,
            mock_retrieve_procedure_definition,
            mock_evaluate_evidence,
            mock_sig_algo,
            mock_file_data_gw_error,
            mock_persist_report_gw);

        // Make sure the error starts with the proper message
        kernel_error_starts_with!(&report_result, Kind::ProcessingFailure, Audience::System,
            "Failed to generate assurance report. Could not get file data for signing: ");

        // Make sure the error contains the gateway error message.  There is a possibility that there could be more text than what the error message starts with and the gateway error message, therefore that's why two assertions are used because these are the two pieces of context we want to ensure are there.
        kernel_error_contains!(&report_result, Kind::ProcessingFailure, Audience::System,
            "Could not get file data for signing: the/action-1/evidence/file.txt. File Data Gateway Error");
    }

    #[test]
    fn persist_report_gateway_error( ) {
        let request = generate_valid_request();

        let report_result = evaluate_and_report(
            &request,
            mock_retrieve_directory_path,
            mock_retrieve_procedure_definition,
            mock_evaluate_evidence,
            mock_sig_algo,
            mock_file_data_gw,
            mock_persist_report_gw_error);

        // Make sure the error starts with the proper message
        kernel_error_starts_with!(&report_result, Kind::GatewayError, Audience::System,
            "Failed to persist the assurance report document. ");

        // Make sure the error contains the gateway error message.  There is a possibility that there could be more text than what the error message starts with and the gateway error message, therefore that's why two assertions are used because these are the two pieces of context we want to ensure are there.
        kernel_error_contains!(&report_result, Kind::GatewayError, Audience::System,
            "Could not persist the assurance report");
    }

   fn mock_retrieve_directory_path(_dir_key: &str) -> Result<String, Error> {
       if _dir_key == "home" {
           return Ok(String::from("/User/procedure-root"))
       } else {
           Ok(String::from("the/directory/path"))
       }

    }

    fn mock_retrieve_directory_path_home_error(dir_key: &str) -> Result<String, Error> {
        if dir_key == "assurance-procedure-file" {
            return Ok(String::from("the/directory/path"))
        }
        Err(Error::for_system(Kind::GatewayError, "Home Path Error".to_string()))
    }

    fn mock_retrieve_directory_path_procedure_definition_doc_error(dir_key: &str) -> Result<String, Error> {
        if dir_key == "home" {
            return Ok(String::from("the/directory/path"))
        }
        Err(Error::for_system(Kind::GatewayError, "Procedure Doc Error".to_string()))
    }

    fn mock_retrieve_procedure_definition(_file_path: &str) -> Result<AssuranceProcedure, Error> {
        Ok(generate_procedure_definition())
    }

    fn mock_retrieve_procedure_definition_error(_file_path: &str) -> Result<AssuranceProcedure, Error> {
        Err(Error::for_system(Kind::GatewayError, "Could not get procedure definition".to_string()))
    }

    fn mock_evaluate_evidence(_files: &EvaluationFiles) -> Result<EvaluationResults, Error> {
        Ok(generate_evaluation_results())
    }

    fn mock_evaluate_evidence_error(_files: &EvaluationFiles) -> Result<EvaluationResults, Error> {
        Err(Error::for_system(Kind::GatewayError, "Could not evaluate evidence files".to_string()))
    }

    fn mock_sig_algo_error(_file_data: &Vec<u8>) -> Result<Signature, Error> {
        Err(Error::for_system(Kind::InvalidInput, "Signature Algorithm Error".to_string()))
    }

    fn mock_file_data_gw_error(_file_path: &str) -> Result<Vec<u8>, Error> {
        Err(Error::for_system(Kind::GatewayError, "File Data Gateway Error".to_string()))
    }

    fn mock_persist_report_gw(_report: &dyn AssuranceReport, _report_directory: &str) -> Result<FilePath, Error> {
        Ok(FilePath::try_from("the/report/file.txt").unwrap())
    }

    fn mock_persist_report_gw_error(_report: &dyn AssuranceReport, _report_directory: &str) -> Result<FilePath, Error> {
        Err(Error::for_system(Kind::GatewayError, "Could not persist the assurance report".to_string()))
    }

}



mod assurance_report_builder {
    use super::*;
    use nape_kernel::values::specification::traits::AssuranceReport;

    #[test]
    fn success() {

        let request = generate_valid_request();
        let procedure_definition = generate_procedure_definition();
        let evaluation_results = generate_evaluation_results();

        let home_dir = &FilePath::from("/User/procedure-root");
        let report_result = AssuranceReportBuilder::new()
            .with_home_dir(&home_dir)
            .with_request(&request)
            .with_definition(&procedure_definition)
            .with_results(&evaluation_results)
            .with_signature_algorithm(mock_sig_algo)
            .with_file_data_gateway(mock_file_data_gw)
            .try_build();

        is_ok!(&report_result);

        let report = report_result.unwrap();

        assert_eq!(report.api_version(), APIVersion::new(1, 0, 0));
        assert_eq!(report.kind(), kind::Kind::AssuranceReport);
        assert_eq!(report.metadata().get("key").unwrap(), "value");
        assert_eq!(report.subject().id.value, "123456789");
        assert_eq!(report.subject().nrn.value, "nrn:sourcecode::example");
        assert_eq!(report.procedure().directory, "some/directory/location");
        assert_eq!(report.procedure().repository, "https://github.com/nape-central");
        assert_eq!(report.summary().activity_count, 1);
        assert_eq!(report.summary().action_count, 2);
        assert_eq!(report.summary().actions_run, 2);
        assert_eq!(report.summary().pass, 2);
        assert_eq!(report.summary().fail, 0);
        assert_eq!(report.summary().inconclusive, 0);
        assert_eq!(report.summary().outcome, Outcome::PASS);
        assert_eq!(report.activities().action_count(), 2);

        let first_activity = report.activities().list().get(0).unwrap();
        assert_eq!(first_activity.name.value, "procedure-1");

        let first_action = first_activity.actions.get(0).unwrap();
        assert_eq!(first_action.name().value, "action-1");
        assert_eq!(first_action.outcome(), &Outcome::PASS);
        assert_eq!(first_action.reason().value, "The test passed");
        assert_eq!(first_action.test_file().file().as_str(), "the/action-1/test/file.py");
        assert_eq!(first_action.test_file().signature().to_string(), "the-signature");
        assert_eq!(first_action.evidence_file().file().as_str(), "the/action-1/evidence/file.txt");
        assert_eq!(first_action.evidence_file().signature().to_string(), "the-signature");

        let second_action = first_activity.actions.get(1).unwrap();
        assert_eq!(second_action.name().value, "action-2");
        assert_eq!(second_action.outcome(), &Outcome::PASS);
        assert_eq!(second_action.reason().value, "The test passed");
        assert_eq!(second_action.test_file().file().as_str(), "the/action-2/test/file.py");
        assert_eq!(second_action.test_file().signature().to_string(), "the-signature");
        assert_eq!(second_action.evidence_file().file().as_str(), "the/action-2/evidence/file.txt");
        assert_eq!(second_action.evidence_file().signature().to_string(), "the-signature");

    }

    #[test]
    fn no_request_error() {
        let procedure_definition =generate_procedure_definition();
        let evaluation_results = generate_evaluation_results();

        let report_result = AssuranceReportBuilder::new()
            .with_definition(&procedure_definition)
            .with_results(&evaluation_results)
            .with_signature_algorithm(mock_sig_algo)
            .with_file_data_gateway(mock_file_data_gw)
            .try_build();

        kernel_error_eq!(report_result, Kind::InvalidInput, Audience::System, "An Evaluation Request was not provided.");

    }

    #[test]
    fn no_definition_error() {
        let request = generate_valid_request();
        let evaluation_results = generate_evaluation_results();

        let report_result = AssuranceReportBuilder::new()
            .with_request(&request)
            .with_results(&evaluation_results)
            .with_signature_algorithm(mock_sig_algo)
            .with_file_data_gateway(mock_file_data_gw)
            .try_build();

        kernel_error_eq!(report_result, Kind::InvalidInput, Audience::System, "A Procedure Definition was not provided.");

    }

    #[test]
    fn no_results_error() {
        let request = generate_valid_request();
        let procedure_definition =generate_procedure_definition();

        let report_result = AssuranceReportBuilder::new()
            .with_request(&request)
            .with_definition(&procedure_definition)
            .with_signature_algorithm(mock_sig_algo)
            .with_file_data_gateway(mock_file_data_gw)
            .try_build();

        kernel_error_eq!(report_result, Kind::InvalidInput, Audience::System, "Evaluation Results were not provided.");

    }

    #[test]
    fn no_signature_algorithm_error() {
        let request = generate_valid_request();
        let procedure_definition =generate_procedure_definition();
        let evaluation_results = generate_evaluation_results();

        let report_result = AssuranceReportBuilder::new()
            .with_request(&request)
            .with_definition(&procedure_definition)
            .with_results(&evaluation_results)
            .with_file_data_gateway(mock_file_data_gw)
            .try_build();

        kernel_error_eq!(report_result, Kind::InvalidInput, Audience::System, "A Signature Algorithm was not provided.");

    }

    #[test]
    fn signature_algo_error() {
        let request = generate_valid_request();
        let procedure_definition =generate_procedure_definition();
        let evaluation_results = generate_evaluation_results();

        let report_result = AssuranceReportBuilder::new()
            .with_home_dir(&FilePath::from("/User/procedure-root"))
            .with_request(&request)
            .with_definition(&procedure_definition)
            .with_results(&evaluation_results)
            .with_signature_algorithm(|_file_data| Err(Error::for_system(Kind::InvalidInput, "Signature Algorithm Error".to_string())))
            .with_file_data_gateway(mock_file_data_gw)
            .try_build();

        kernel_error_eq!(report_result, Kind::ProcessingFailure, Audience::System, "Failed to sign the file: the/action-1/evidence/file.txt. Signature Algorithm Error");
    }

    #[test]
    fn no_file_data_gateway_error() {
        let request = generate_valid_request();
        let procedure_definition =generate_procedure_definition();
        let evaluation_results = generate_evaluation_results();

        let report_result = AssuranceReportBuilder::new()
            .with_home_dir(&FilePath::from("/User/procedure-root"))
            .with_request(&request)
            .with_definition(&procedure_definition)
            .with_results(&evaluation_results)
            .with_signature_algorithm(mock_sig_algo)
            .try_build();

        kernel_error_eq!(report_result, Kind::InvalidInput, Audience::System, "A File Data Gateway was not provided.");

    }

    #[test]
    fn file_data_gateway_error() {
        let request = generate_valid_request();
        let procedure_definition =generate_procedure_definition();
        let evaluation_results = generate_evaluation_results();

        let report_result = AssuranceReportBuilder::new()
            .with_home_dir(&FilePath::from("/User/procedure-root"))
            .with_request(&request)
            .with_definition(&procedure_definition)
            .with_results(&evaluation_results)
            .with_signature_algorithm(mock_sig_algo)
            .with_file_data_gateway(|_file_path| Err(Error::for_system(Kind::InvalidInput, "File Data Gateway Error".to_string())))
            .try_build();

        kernel_error_eq!(report_result, Kind::ProcessingFailure, Audience::System, "Could not get file data for signing: the/action-1/evidence/file.txt. File Data Gateway Error");
    }

}

fn generate_valid_request() -> EvaluateEvidence {
    let mut metadata: Vec<(String, String)> = Vec::new();
    metadata.push(("key".to_string(), "value".to_string()));

    EvaluateEvidence::builder()
        .subject_id("123456789")
        .subject_nrn("nrn:sourcecode::example")
        .procedure_repository("https://github.com/nape-central")
        .procedure_directory("some/directory/location")
        .metadata(&metadata)
        .try_build().unwrap()
}

fn generate_procedure_definition() -> AssuranceProcedure {

    let action1 = Action::builder().name("action-1").short_description("action-1 short").long_description("action-1 long").test_file_path("the/action-1/test/file.py").evidence_file_path("the/action-1/evidence/file.txt").try_build().unwrap();

    let action2 = Action::builder().name("action-2").short_description("action-2 short").long_description("action-2 long").test_file_path("the/action-2/test/file.py").evidence_file_path("the/action-2/evidence/file.txt").try_build().unwrap();

    let activity = Activity::new("procedure-1", "Short Desc", "Long Desc").unwrap()
        .add(action1)
        .add(action2);

    AssuranceProcedure::builder()
        .api_version("1.0.0")
        .procedure_info("nrn:sourcecode::example", "A Short Desc.", "This is an example procedure")
        .add_activity(&activity)
        .try_build().unwrap()
}

fn generate_evaluation_results() -> EvaluationResults {
    let results = EvaluationResults::default();

    // REMEMBER - the EvaluateionResults will containt the home_root plue the test/evidence path in the procedure.
    let results = results.add_result(
        &FilePath::try_from("/User/procedure-root/the/action-1/evidence/file.txt").unwrap(),
        &FilePath::try_from("/User/procedure-root/the/action-1/test/file.py").unwrap(),
        TestResult::try_from("pass", "The test passed").unwrap());
    let results= results.add_result(
        &FilePath::try_from("/User/procedure-root/the/action-2/evidence/file.txt").unwrap(),
        &FilePath::try_from("/User/procedure-root/the/action-2/test/file.py").unwrap(),
        TestResult::try_from("pass", "The test passed").unwrap());

    results
}

fn mock_sig_algo(_file_data: &Vec<u8>) -> Result<Signature, Error> {
    Signature::try_new(SignatureType::SHA256, "the-signature")
}

fn mock_file_data_gw(_file_path: &str) -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}


