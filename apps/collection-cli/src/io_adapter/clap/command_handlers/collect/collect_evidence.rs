use clap::ArgMatches;
use nape_domain::evidence_collection::usecases::collect_evidence::usecase::{CollectEvidenceRequest, UCCollectEvidenceFile};
use nape_kernel::error::{Error, Kind};
use crate::io_adapter::clap::command_handler_boundary::CommandHandlerBoundary;

pub struct CollectEvidenceCommandHandler<'a> {
    pub command_name: &'a str,
    usecase: UCCollectEvidenceFile,
}

impl<'a> CollectEvidenceCommandHandler<'a> {
    pub fn new(usecase: UCCollectEvidenceFile) -> CollectEvidenceCommandHandler<'a> {
        CollectEvidenceCommandHandler { command_name: "evidence", usecase }
    }
}

impl<'a> CommandHandlerBoundary for CollectEvidenceCommandHandler<'a> {
    fn name(&self) -> &str {
        self.command_name
    }
    fn handle(&self, args: &ArgMatches) -> Result<(), Error> {
        let request = extract_arguments(args)?;
        let _ = (self.usecase)(&request)?;
        Ok(())
    }
}

fn extract_arguments(matches: &ArgMatches)  -> Result<CollectEvidenceRequest, Error> {

    let action_name = matches.get_one::<String>("control-activity-name")
        .ok_or(Error::for_user(Kind::InvalidInput, String::from("The control action name is required.")))?;
    let file_path = matches.get_one::<String>("evidence-file-path")
        .ok_or(Error::for_user(Kind::InvalidInput, String::from("The evidence file path is required.")))?;
    let file_name = matches.get_one::<String>("evidence-file-name");

    Ok(CollectEvidenceRequest {
        action_name, file_path,
        file_name: file_name.as_ref().map(|arg0: &&String| String::as_str(*arg0)),
    })

}
