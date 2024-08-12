use clap::ArgMatches;
use nape_kernel::error::Error;
use crate::io_adapter::clap::command_handler_boundary::CommandHandlerBoundary;

pub struct CollectCommandHandler<'a> {
    command_name: &'a str,
    subcommands: Vec<Box<dyn CommandHandlerBoundary>>,
}

impl<'a> CollectCommandHandler<'a> {
    pub fn new(subcommands: Vec<Box<dyn CommandHandlerBoundary>>) -> CollectCommandHandler<'a> {
        CollectCommandHandler {
            command_name: "collect",
            subcommands
        }
    }
}

impl<'a> CommandHandlerBoundary for CollectCommandHandler<'a> {
    fn name(&self) -> &str {
        self.command_name
    }
    fn handle(&self, args: &ArgMatches) -> Result<(), Error> {
        for subcommand in &self.subcommands {
            if let Some(subcommand_matches) = args.subcommand_matches(subcommand.name()) {
                return subcommand.handle(subcommand_matches);
            }
        }
        Ok(())
    }
}