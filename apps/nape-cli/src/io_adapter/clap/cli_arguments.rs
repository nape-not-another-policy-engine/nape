use clap::{Arg, ArgAction};

pub fn source() -> Arg {
    Arg::new("source")
        .long("source")
        .value_name("DIRECTORY")
        .required(true)
}

pub fn package_purl() -> Arg {
    Arg::new("package")
        .long("package")
        .value_name("PURL")
        .required(true)
}

pub fn dependency_package() -> Arg {
    Arg::new("dependency-package")
        .long("dependency-package")
        .value_name("BUILD_RESULT_DIRECTORY")
        .action(ArgAction::Append)
        .num_args(1)
}

pub fn manifest_digest() -> Arg {
    Arg::new("manifest-digest")
        .long("manifest-digest")
        .value_name("SHA256_DIGEST")
        .required(true)
}

pub fn registry_profile() -> Arg {
    Arg::new("registry-profile")
        .long("registry-profile")
        .value_name("REGISTRY_MAP_FILE")
}

pub fn registry_endpoint() -> Arg {
    Arg::new("registry-endpoint")
        .long("registry-endpoint")
        .value_name("URL")
}

pub fn plan_only() -> Arg {
    Arg::new("plan-only")
        .long("plan-only")
        .action(ArgAction::SetTrue)
        .required(true)
}

pub fn output() -> Arg {
    Arg::new("output")
        .long("output")
        .value_name("NEW_DIRECTORY")
        .required(true)
}

pub fn local_package() -> Arg {
    Arg::new("local-package")
        .long("local-package")
        .value_name("BUILD_RESULT_DIRECTORY")
        .required(true)
}

pub fn verify_package_purl() -> Arg {
    Arg::new("package")
        .long("package")
        .value_name("PURL")
        .requires_all(["manifest-digest", "registry-configuration"])
        .conflicts_with("local-package")
}

pub fn verify_manifest_digest() -> Arg {
    Arg::new("manifest-digest")
        .long("manifest-digest")
        .value_name("SHA256_DIGEST")
        .requires_all(["package", "registry-configuration"])
        .conflicts_with("local-package")
}

pub fn verify_registry_profile() -> Arg {
    registry_profile()
        .requires_all(["package", "manifest-digest"])
        .conflicts_with("local-package")
}

pub fn verify_registry_endpoint() -> Arg {
    registry_endpoint()
        .requires_all(["package", "manifest-digest"])
        .conflicts_with("local-package")
}

pub fn action() -> Arg {
    Arg::new("action")
        .long("action")
        .value_name("ACTIVITY.ACTION")
        .required(true)
}

pub fn evidence_file() -> Arg {
    Arg::new("file")
        .long("file")
        .value_name("SOURCE_FILE")
        .required(true)
}

pub fn evidence_file_name() -> Arg {
    Arg::new("file-name")
        .long("file-name")
        .value_name("DEFINITION_FILE_NAME")
}

pub fn metadata() -> Arg {
    Arg::new("meta")
        .long("meta")
        .value_names(["KEY", "VALUE"])
        .num_args(2)
        .action(ArgAction::Append)
}

pub fn subject_file() -> Arg {
    Arg::new("subject-file")
        .long("subject-file")
        .value_name("FILE")
        .required(true)
}
