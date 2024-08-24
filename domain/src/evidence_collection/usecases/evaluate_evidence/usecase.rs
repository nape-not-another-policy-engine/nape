use nape_kernel::algorithms::signature_algorithm::{SignatureAlgorithm};
use nape_kernel::error::{Error, Kind};
use nape_kernel::gateways::directory_list::RetrieveDirectoryPath;
use nape_kernel::gateways::file_data_gateway::FileDataGateway;
use nape_kernel::values::specification::{assurance_report};
use nape_kernel::values::specification::file_path::FilePath;
use nape_kernel::values::specification::assurance_report::signed_file::SignedFile;
use nape_kernel::values::specification::v1_0_0::assurance_report::AssuranceReportV1;
use nape_kernel::values::specification::v1_0_0::assurance_procedure::AssuranceProcedure;
use crate::evidence_collection::usecases::evaluate_evidence::gateway::{EvaluateEvidenceGateway, PersistReportGateway, RetrieveAssuranceProcedure};
use crate::evidence_collection::usecases::evaluate_evidence::gateway_boundary::combine_paths;
use crate::evidence_collection::usecases::evaluate_evidence::gateway_boundary::request::EvaluationFiles;
use crate::evidence_collection::usecases::evaluate_evidence::gateway_boundary::response::{EvaluationResults, TestResult};
use crate::evidence_collection::usecases::evaluate_evidence::usecase_boundary::request::EvaluateEvidence;

///  # Overview
///
///  An interface for the usecase that takes evaluates a collection of evidence files against a series of control actions within a procedure definition, and updates the [`AssuranceReport`] with the results of the evaluation.
///
///  # Arguments
///
/// * `request` - An [`EvidenceEvaluation`] containing all the necessary evidence and control action data for the usecase to evaluate.
///
/// # Returns
///
///  A [`Result`] of either a [`FilePath`] containing a file path to the report, or an [`Error`].
///
pub type EvaluateAndReportEvidenceUC = fn(request: &EvaluateEvidence) -> Result<FilePath, Error>;


/// # Overview
///
/// The `evaluate_and_report` function is a usecase that takes a request to evaluate a collection of evidence files against a series of control actions within a procedure definition, creates a [`ProcedureAssuranceReport`] with the results of the evaluation, and persists the report.
///
/// # Arguments
///
/// * `&request` - A reference to an [`EvidenceEvaluation`] request containing all the necessary evidence and control action data for the usecase to evaluate.
/// * `retrieve_path` - An implementation of the [`RetrieveDirectoryPath`] gateway.
/// * `retrieve_definition` - An implementation of the [`RetrieveAssuranceProcedure`] gateway.
/// * `evaluate_evidence` -An implementation of the [`EvaluateEvidenceGateway`] gateway.
/// * `signature_algorithm` - An implementation of the [`SignatureAlgorithm`] gateway.
/// * `file_data_gateway` - An implementation of the [`FileDataGateway`] gateway.
/// * `persist_report` - An implementation of the [`PersistReportGateway`] gateway.
///
/// # Returns
///
/// - A [`Result`] of either a [`ProcedureAssuranceReport`] containing the results of the evaluation, or an [`Error`].
///  - All [`Error`]s are for [`Audience::System`]
///  - There are two [ `Kind`]s of [`Error`] that can be returned: [`Kind::GatewayError`] and [`Kind::ProcessingFailure`]
///
pub fn evaluate_and_report(
    request: &EvaluateEvidence,
    retrieve_path: RetrieveDirectoryPath,
    retrieve_definition: RetrieveAssuranceProcedure,
    evaluate_evidence: EvaluateEvidenceGateway,
    signature_algorithm: SignatureAlgorithm,
    file_data_gateway: FileDataGateway,
    persist_report: PersistReportGateway) -> Result<FilePath, Error> {

    let definition_path = retrieve_path("assurance-procedure-file")
        .map_err(|error| Error::for_system(Kind::GatewayError,
                                           format!("Failed to retrieve the 'assurance-procedure-file' path. {}", error.message)))?;

    let home_dir = retrieve_path("home")
        .map_err(|error| Error::for_system(Kind::GatewayError,
                                           format!("Failed to retrieve the 'home' directory path. {}", error.message)))?;

    let procedure = retrieve_definition(&definition_path)
        .map_err(|error| Error::for_system(Kind::GatewayError,
                                           format!("Failed to retrieve procedure definition. {}", error.message)))?;

    let home_root = FilePath::from(&home_dir);
    let evaluation_files = EvaluationFiles::from(&home_root, &procedure)
        .map_err(|error| Error::for_system(Kind::GatewayError,
                                           format!("Failed to create evaluation files. {}", error.message)))?;

    let evaluation_results = evaluate_evidence(&evaluation_files)
        .map_err(|error| Error::for_system(Kind::GatewayError,
                                           format!("Failed to evaluate evidence files. {}", error.message)))?;

    let report = AssuranceReportBuilder::new()
        .with_home_dir(&home_root)
        .with_results(&evaluation_results)
        .with_definition(&procedure)
        .with_request(&request)
        .with_signature_algorithm(signature_algorithm)
        .with_file_data_gateway(file_data_gateway)
        .try_build()
        .map_err(|error| Error::for_system(Kind::ProcessingFailure,
                                           format!("Failed to generate assurance report. {}", error.message)))?;

    let report_path = persist_report(&report, &home_dir).map_err(|error|
        Error::for_system(Kind::GatewayError,
                          format!("Failed to persist the assurance report document. {}", error.message)))?;

    Ok(report_path)

}

pub struct AssuranceReportBuilder<'a> {
    home_dir: Option<FilePath>,
    request: Option<&'a EvaluateEvidence>,
    procedure_definition: Option<&'a AssuranceProcedure>,
    evaluation_results: Option<&'a EvaluationResults>,
    file_data_gateway: Option<FileDataGateway>,
    signature_algorithm: Option<SignatureAlgorithm>,
}

impl<'a> AssuranceReportBuilder<'a> {
    pub fn new() -> Self {
        Self {
            home_dir: None,
            request: None,
            procedure_definition: None,
            evaluation_results: None,
            file_data_gateway: None,
            signature_algorithm: None,
        }
    }

    // todo - create unit test for this
    pub fn with_home_dir(&mut self, home_dir: &FilePath) -> &mut Self {
        self.home_dir = Some(home_dir.clone());
        self
    }
    pub fn with_request(&mut self, request: &'a EvaluateEvidence) -> &mut Self {
        self.request = Some(request);
        self
    }

    pub fn with_definition(&mut self, procedure_definition: &'a AssuranceProcedure) -> &mut Self {
        self.procedure_definition = Some(procedure_definition);
        self
    }

    pub fn with_results(&mut self, evaluation_results: &'a EvaluationResults) -> &mut Self {
        self.evaluation_results = Some(evaluation_results);
        self
    }

    pub fn with_signature_algorithm(&mut self, signature_algorithm: SignatureAlgorithm) -> &mut Self {
        self.signature_algorithm = Some(signature_algorithm);
        self
    }

    pub fn with_file_data_gateway(&mut self, file_data_gateway: FileDataGateway) -> &mut Self {
        self.file_data_gateway = Some(file_data_gateway);
        self
    }

    pub fn try_build(&self) -> Result<AssuranceReportV1, Error> {

        let definition = self.validate_definition()?;
        let request = self.validate_request()?;
        let results = self.validate_results()?;
        let signer = self.signature_algorithm
            .ok_or_else(|| Error::for_system(Kind::InvalidInput, String::from("A Signature Algorithm was not provided.")))?;
        let file_data_gw = self.file_data_gateway
            .ok_or_else(|| Error::for_system(Kind::InvalidInput, String::from("A File Data Gateway was not provided.")))?;
        let home = self.home_dir.as_ref()
            .ok_or_else(|| Error::for_system(Kind::InvalidInput, String::from("A Home Directory was not provided.")))?;
        let activities = try_create_report_activities(home, &definition, &results, file_data_gw, signer)?;

        let final_report = AssuranceReportV1::builder()
            .use_metadata(&request.metadata())
            .use_subject(&request.subject())
            .use_procedure(&request.procedure())
            .use_activities(&activities)
            .try_build()?;

        Ok(final_report)
    }

    fn validate_definition(&self) -> Result<&AssuranceProcedure, Error> {
        self.procedure_definition.clone()
            .ok_or_else(|| Error::for_system(Kind::InvalidInput, String::from("A Procedure Definition was not provided.")))
    }

    fn validate_request(&self) -> Result<&EvaluateEvidence, Error> {
        self.request.clone()
            .ok_or_else(|| Error::for_system(Kind::InvalidInput, String::from("An Evaluation Request was not provided.")))
    }

    fn validate_results(&self) -> Result<&EvaluationResults, Error> {
        self.evaluation_results.clone()
            .ok_or_else(|| Error::for_system(Kind::InvalidInput, String::from("Evaluation Results were not provided.")))
    }

}



fn try_create_report_activities(
    home: &FilePath,
    definition: &AssuranceProcedure,
    results: &EvaluationResults,
    file_data_gateway: FileDataGateway,
    signature_algorithm: SignatureAlgorithm) -> Result< assurance_report::activities::Activities, Error> {

    let mut builder = assurance_report::activities::Activities::builder();

    for definition_activity in &definition.activities.list {
        // TODO - Move home the combine_paths onto the try_get_test_result, and try_create_signed_file functions so you can combine to retrive the file data, but record in the assurance report as the non-canonical path
        for definition_action in &definition_activity.actions {
            let test_result = try_get_test_result(results, home, &definition_action.evidence, &definition_action.test)?;
            let signed_evidence = try_create_signed_file(home, &definition_action.evidence, file_data_gateway, signature_algorithm)?;
            let signed_test = try_create_signed_file(home, &definition_action.test, file_data_gateway, signature_algorithm)?;
            // TODO - REMOVE ONCE - Testing of Canonical paths works
            // let test_result = try_get_test_result(results, &definition_action.evidence, &definition_action.test)?;
            // let signed_evidence = try_create_signed_file(&definition_action.evidence, file_data_gateway, signature_algorithm)?;
            // let signed_test = try_create_signed_file(&definition_action.test, file_data_gateway, signature_algorithm)?;
            let report_action = assurance_report::action::Action::builder()
                .use_name(&definition_action.name)
                .use_outcome(&test_result.outcome)
                .use_reason(&test_result.reason)
                .use_test_file_signature(&signed_test)
                .use_evidence_file_signature(&signed_evidence)
                .try_build()?;

            builder.add_action(&definition_activity.name.value, &report_action);
        }

    }

    let report_activities = builder.try_build()?;

    Ok(report_activities)
}
fn try_get_test_result(results: &EvaluationResults, home_root: &FilePath, evidence: &FilePath, test: &FilePath) -> Result<TestResult, Error> {
    let canonical_test_path = combine_paths(home_root, test)?;
    let canonical_evidence_path = combine_paths(home_root, evidence)?;
    results.result_for(&canonical_evidence_path, &canonical_test_path)
        .ok_or_else(|| Error::for_system(Kind::InvalidInput, format!("No test result found for evidence: {:?} and test: {:?}", evidence, test)))
}

fn try_create_signed_file(home_root: &FilePath, file_path:  &FilePath, file_data_gateway: FileDataGateway, signature_algorithm: SignatureAlgorithm) -> Result<SignedFile, Error> {

    let canonical_path = combine_paths(home_root, file_path)?;

    // Use Canonical Path to get the file data
    let file_data = file_data_gateway(&canonical_path.as_str())
        .map_err(|error| Error::for_system(Kind::ProcessingFailure,
                                         format!("Could not get file data for signing: {}. {}", file_path.as_str(), error)))?;

    let signature_result = signature_algorithm(&file_data)
        .map_err(|error| Error::for_system(Kind::ProcessingFailure,
                                         format!("Failed to sign the file: {}. {}", file_path.as_str(), error)))?;

    // use the non-canonical path to create the signed file because the non-canonical path is the path that is provided in the assurance procedure
    let signed_file = SignedFile::new(&file_path.as_str(), &signature_result)
        .map_err(|error| Error::for_system(Kind::ProcessingFailure,
                                         format!("Could not create a signed file for: {}. {}", file_path.as_str(), error)))?;

    Ok(signed_file)
}