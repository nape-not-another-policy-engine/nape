use clap::{Command};
use crate::io_adapter::clap::cli_arguments::{control_action_name, evidence_file_name, evidence_file_path, metadata, procedure_directory, procedure_link, subject, subject_id};

pub fn collect() -> Command {
    Command::new("collect")
        .about("Collects evidence, applies test of details, generates report, and uploads results to your evidence repository.")
        .subcommand(start())
        .subcommand(evidence())
        .subcommand(report())
}


pub fn start() -> Command {
    Command::new("start")
        .about("Starts a new collection procedure by retrieving a NAPE Procedure Definition and setting up local directories for the collection of evidence.")
        .arg(subject())
        .arg(subject_id())
        .arg(procedure_link())
        .arg(procedure_directory())
        .arg(metadata())
}

pub fn evidence() -> Command {
    Command::new("evidence")
        .about("Collect a piece of evidence and associate it with a Control Action.")
        .arg(control_action_name())
        .arg(evidence_file_path())
        .arg(evidence_file_name())
}

pub fn report() -> Command {
    Command::new("report")
        .about("Evaluate all of the collected evidence and generate a report.")
}