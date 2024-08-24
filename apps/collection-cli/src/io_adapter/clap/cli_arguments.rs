use clap::{Arg, ArgAction, value_parser};

pub fn subject() -> Arg {
    Arg::new("subject")
        .short('s')
        .long("subject")
        .value_name("NAPE Resource Name")
        .help("The subject is the NAPE Resource Name (NRN) that the assurance procedure will be run against.")
        .required(true)
}

pub fn subject_id() -> Arg {
    Arg::new("subject-id")
        .short('i')
        .long("subject-id")
        .value_name("Subject Identifier")
        .help("The string-based ID representing and instance of the procedure.")
        .required(true)
}

pub fn procedure_link() -> Arg {
    Arg::new("procedure-link")
        .short('l')
        .long("procedure-link")
        .value_name("NAPE Assurance Procedure Definition Link")
        .help("The URL to the NAPE Repository containing the NAPE Assurance Procedure Definition and all related NAPE Activity & Action Tests.")
        .required(true)
}

pub fn procedure_directory() -> Arg {
    Arg::new("procedure-directory")
        .short('d')
        .long("procedure-directory")
        .value_name("NAPE Assurance Procedure Definition Directory")
        .help("The directory within the NAPE Repository which contains NAPE Assurance Procedure Definition and all related NAPE Activity & Action Tests for the assurance procedure you want to run.")
        .required(true)
}

pub fn metadata() -> Arg {
    Arg::new("metadata")
        .action(ArgAction::Append)
        .short('m')
        .long("meta")
        .help("Metadata that provides additional context.")
        .num_args(2)
        .value_name("Metadata")
        .value_parser(value_parser!(String))
        .value_delimiter(' ')
}

pub fn control_action_name() -> Arg {
    Arg::new("control-activity-name")
        .short('a')
        .long("control-activity")
        .value_name("Control Activity Name")
        .help("The name of the control activity that the evidence is associated with.")
        .required(true)
}

pub fn evidence_file_path() -> Arg {
    Arg::new("evidence-file-path")
        .short('f')
        .long("file-path")
        .value_name("Evidence File Path")
        .help("The path to the file that contains the evidence.")
        .required(true)
}

pub fn evidence_file_name() -> Arg {
    Arg::new("evidence-file-name")
        .short('n')
        .long("file-name")
        .value_name("Evidence File Name")
        .help("The a file name and type that you'd like to rename the evidence file to.  This is optional and usefule when the control activity expects a file by a specific name, although the file is currently stored as a different name.")
        .required(false)
}