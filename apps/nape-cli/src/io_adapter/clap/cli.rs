use crate::io_adapter::clap::cli_commands;
use clap::{command, ArgMatches};
use kernel_oss::error::Error;

pub fn run() -> Result<ArgMatches, Error> {
    Ok(command!()
        .name("NAPE CLI").version("1.0.0")
        .propagate_version(true)
        .about("Collects evidence, applies test of details, generates report, and uploads results to your repository.")
        .subcommand(cli_commands::collect())
        .get_matches())
}
