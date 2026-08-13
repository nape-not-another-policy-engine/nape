use crate::io_adapter::clap::cli_commands;
use clap::{command, ArgMatches};

pub fn run() -> ArgMatches {
    command!()
        .name("NAPE CLI").version("2.0.0")
        .propagate_version(true)
        .about("Builds and publishes Attestify packages, executes Verification Procedure V2, and persists local Verification results.")
        .subcommand_required(true)
        .subcommand(cli_commands::package())
        .subcommand(cli_commands::start())
        .subcommand(cli_commands::evidence())
        .subcommand(cli_commands::verify())
        .get_matches()
}
