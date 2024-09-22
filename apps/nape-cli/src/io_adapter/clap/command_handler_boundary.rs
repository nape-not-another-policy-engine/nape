use clap::ArgMatches;
use nape_kernel::error::Error;

// This was an idea for eac handler to give it this such that this can be passed into the cli
pub trait CommandHandlerBoundary {
    fn name(&self) -> &str;

    fn handle(&self, args: &ArgMatches) -> Result<(), Error>;
}