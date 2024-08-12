mod gateway_adapter;
mod io_adapter;
mod state_management;
mod usecase_configuration;

mod filesystem_state_configuration;

use clap::{ArgMatches};
use nape_kernel::error::{Error};
use crate::io_adapter::clap::{cli};
use crate::io_adapter::clap::command_handler_boundary::CommandHandlerBoundary;
use crate::io_adapter::clap::command_handlers::collect::collect_command_handler::CollectCommandHandler;
use crate::io_adapter::clap::command_handlers::collect::collect_evidence::CollectEvidenceCommandHandler;
use crate::io_adapter::clap::command_handlers::collect::collect_report::EvaluateAndReportCommandHandler;
use crate::io_adapter::clap::command_handlers::collect::collect_start::StartCollectionCommandHandler;
use crate::usecase_configuration::{collect_evidence, evidence_report, start_collection};


fn main() {
     let _ = cli::run()
         .map_err(|e| eprintln!("{}", e))
         .map(|command_results| handle_command_results(&command_results).map_err(|e| eprintln!(" {}", e))
         );
}

fn handle_command_results(matches: &ArgMatches) -> Result<(), Error>{

    let collect_command_handler = configure_collect_command_handler();

    match matches.subcommand() {
        Some(("collect", args)) => { collect_command_handler.handle(args) },
        _ => { Ok(()) }
    }

}

fn configure_collect_command_handler() -> CollectCommandHandler<'static> {

    // #1 - Instantiate injectable dependencies here
    let uc_start_collection = start_collection::factory_std_fs_git2();
    let uc_evidence_collection = collect_evidence::std_fs_factory();
    let uc_evidence_report = evidence_report::std_fs_factory();

    // #2 - Instantiate the subcommand handlers here
    let start_collection_subcommand = StartCollectionCommandHandler::new(uc_start_collection);
    let evidence_collection_subcommand = CollectEvidenceCommandHandler::new(uc_evidence_collection);
    let evidence_report_subcommand = EvaluateAndReportCommandHandler::new(uc_evidence_report);

    // #3 - Instantiate the command handler here
    CollectCommandHandler::new(
        vec![
            Box::new(start_collection_subcommand),
            Box::new(evidence_collection_subcommand),
            Box::new(evidence_report_subcommand)
        ])
}
