use nape_kernel::error::Error;
use nape_kernel::values::specification::file_path::FilePath;
use nape_kernel::values::specification::traits::AssuranceReport;
use nape_kernel::values::specification::v1_0_0::assurance_procedure::AssuranceProcedure;
use crate::evidence_collection::usecases::evaluate_evidence::gateway_boundary::request::EvaluationFiles;
use crate::evidence_collection::usecases::evaluate_evidence::gateway_boundary::response::EvaluationResults;

/// # Overview
///
/// An interface for the gateway which retrieves the [`AssuranceProcedure`] from an assurance procedure serialized as a file.
///
/// # Arguments
///
/// * `file_path` - The file path of the procedure definition to be retrieved.
///
/// # Returns
///
/// A [`Result`] of either a [`AssuranceProcedure`] or an [`Error`].
///
///  # Design Decision
///
/// Date: Wednesday, June 19, 2024 | By: Bill Bensing
///
///  * This interface does not explicitly depend upon context to determine which procedure definition to return. The current gateway design assumes the underlying implementation will contain proper context to identify the procedure definition.
/// * In the future, this gateway interface can be updated to include an argument or context dependency which identifies the procedure definition to be returned.  This may happen when enough evidence suggests that the current design is not sufficient for the needs of the system.
///
pub type RetrieveAssuranceProcedure = fn(file_path: &str) -> Result<AssuranceProcedure, Error>;

///  # Overview
///
/// An interface for the gateway which handles the individual evaluation of an evidence file against a series of control actions
///
/// # Arguments
///
/// * `files` - An [`EvaluationFiles`] containing all the necessary evidence and control action data for the gateway to evaluate.
///
/// # Returns
///
/// A [`Result`] of either an [`EvaluationResults`] containing the results of the evaluation, or an [`Error`].
///
pub type EvaluateEvidenceGateway = fn (files: &EvaluationFiles) -> Result<EvaluationResults, Error>;

/// # Overview
///
/// The [`PersistReportGateway`] is a function that takes an [`AssuranceReportV1`]  and a directory path where the report will be created by serailizing it into a consumable asset.
///
/// # Arguments
///
/// * `report` - The [`AssuranceReportV1`] to be persisted.
/// * `report_directory` - The directory where the report will be persisted.
///
/// # Returns
///
/// A [`Result`] of either a [`FilePath`] containing the location of the report, or an [`Error`].
///
/// # Design Decision
///
///  * This gateway_adapter does not assume the format of the report, only that it will be persisted in the directory provided, given the file name.
///
pub type PersistReportGateway = fn(report: &dyn AssuranceReport, report_directory: &str) -> Result<FilePath, Error>;
