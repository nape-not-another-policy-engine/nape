use nape_domain::evidence_collection::usecases::evaluate_evidence::usecase::{evaluate_and_report, EvaluateAndReportEvidenceUC};
use nape_domain::evidence_collection::usecases::evaluate_evidence::usecase_boundary::request::EvaluateEvidence;
use nape_kernel::error::Error;
use nape_kernel::values::specification::file_path::FilePath;
use crate::gateway_adapter::nape_evaluator::evaluate_evidence_gateway::nape_evidence_evaluator;
use crate::gateway_adapter::serde::persist_report_gateway::save_report_as_yaml;
use crate::gateway_adapter::sha2::signature_algorithm::sha256_signature;
use crate::gateway_adapter::state_management::retrieve_directory_path::directory_path_from_app_state;
use crate::gateway_adapter::std_fs::file_data_gateway::read_file_data;
use crate::gateway_adapter::std_fs::retrieve_assurance_procedure::from_yaml_on_filesystem;

pub fn std_fs_factory() -> EvaluateAndReportEvidenceUC {
    move |request: &EvaluateEvidence| -> Result<FilePath, Error> {
        evaluate_and_report(request,
                            directory_path_from_app_state,
                            from_yaml_on_filesystem,
                            nape_evidence_evaluator,
                            sha256_signature,
                            read_file_data,
                            save_report_as_yaml
        )
    }
}