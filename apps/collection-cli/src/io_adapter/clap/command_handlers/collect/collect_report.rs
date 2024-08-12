use clap::ArgMatches;
use nape_domain::evidence_collection::usecases::evaluate_evidence::usecase::EvaluateAndReportEvidenceUC;
use nape_domain::evidence_collection::usecases::evaluate_evidence::usecase_boundary::request::EvaluateEvidence;
use nape_kernel::error::{Error, Kind};
use crate::gateway_adapter::state_management::retrieve_app_state::app_state_from_nape_config;
use crate::io_adapter::clap::command_handler_boundary::CommandHandlerBoundary;

pub struct EvaluateAndReportCommandHandler<'a> {
    pub command_name: &'a str,
    usecase: EvaluateAndReportEvidenceUC,
}

impl<'a> EvaluateAndReportCommandHandler<'a> {
    pub fn new(usecase: EvaluateAndReportEvidenceUC) -> EvaluateAndReportCommandHandler<'a> {
        EvaluateAndReportCommandHandler { command_name: "report", usecase }
    }
}

impl<'a> CommandHandlerBoundary for EvaluateAndReportCommandHandler<'a> {
    fn name(&self) -> &str {
        self.command_name
    }
    fn handle(&self, _args: &ArgMatches) -> Result<(), Error> {
        let request = create_request()?;
        match (self.usecase)(&request) {
            Ok(_) => { Ok(()) },
            Err(e) => Err(e)
        }
    }
}

fn create_request()  -> Result<EvaluateEvidence, Error> {

    let app_state = app_state_from_nape_config()?;

    EvaluateEvidence::builder()
        .subject_nrn(&app_state.subject_nrn)
        .subject_id(&app_state.subject_id)
        .procedure_directory(&app_state.procedure_directory)
        .procedure_repository(&app_state.procedure_repository)
        .metadata(&app_state.metadata.into_iter().collect())
        .try_build()
        .map_err(|error| Error::for_system(Kind::InvalidInput,
                                           format!("Failed to build the EvaluateEvidence request for the command 'collect report'. {}", error.message)))
}