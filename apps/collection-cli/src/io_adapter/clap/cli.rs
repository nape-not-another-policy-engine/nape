use clap::{ArgMatches, command};
use nape_kernel::error::Error;
use crate::io_adapter::clap::cli_commands;

pub fn run() -> Result<ArgMatches, Error> {
    Ok(command!()
        .name("NAPE CLI").version("1.0.0")
        .propagate_version(true)
        .about("Collects evidence, applies test of details, generates report, and uploads results to your repository.")
        .subcommand(cli_commands::collect())
        .get_matches())
}

