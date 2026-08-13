use clap::{ArgGroup, Command};

pub fn package() -> Command {
    Command::new("package")
        .about("Build deterministic Attestify packages.")
        .subcommand_required(true)
        .subcommand(
            Command::new("build")
                .about("Build one deterministic local package result.")
                .arg(super::cli_arguments::source())
                .arg(super::cli_arguments::package_purl())
                .arg(super::cli_arguments::dependency_package())
                .arg(super::cli_arguments::output()),
        )
        .subcommand(
            Command::new("resolve")
                .about("Acquire and verify one exact root package without executing it.")
                .group(
                    ArgGroup::new("registry-configuration")
                        .args(["registry-profile", "registry-endpoint"])
                        .required(true)
                        .multiple(false),
                )
                .arg(super::cli_arguments::package_purl())
                .arg(super::cli_arguments::manifest_digest())
                .arg(super::cli_arguments::registry_profile())
                .arg(super::cli_arguments::registry_endpoint())
                .arg(super::cli_arguments::plan_only()),
        )
        .subcommand(
            Command::new("publish")
                .about("Publish one exact verified local package result.")
                .group(
                    ArgGroup::new("registry-configuration")
                        .args(["registry-profile", "registry-endpoint"])
                        .required(true)
                        .multiple(false),
                )
                .arg(super::cli_arguments::local_package())
                .arg(super::cli_arguments::registry_profile())
                .arg(super::cli_arguments::registry_endpoint()),
        )
}

pub fn start() -> Command {
    Command::new("start")
        .about("Start one NAPE-managed Verification Procedure run.")
        .group(
            ArgGroup::new("procedure-source")
                .args(["local-package", "package"])
                .required(true)
                .multiple(false),
        )
        .group(
            ArgGroup::new("registry-configuration")
                .args(["registry-profile", "registry-endpoint"])
                .multiple(false),
        )
        .arg(super::cli_arguments::local_package().required(false))
        .arg(super::cli_arguments::verify_package_purl())
        .arg(super::cli_arguments::verify_manifest_digest())
        .arg(super::cli_arguments::verify_registry_profile())
        .arg(super::cli_arguments::verify_registry_endpoint())
        .arg(super::cli_arguments::subject_file())
        .arg(super::cli_arguments::metadata())
}

pub fn evidence() -> Command {
    Command::new("evidence")
        .about("Add or replace one Evidence payload in the current run.")
        .arg(super::cli_arguments::action())
        .arg(super::cli_arguments::evidence_file())
        .arg(super::cli_arguments::evidence_file_name())
}

pub fn verify() -> Command {
    Command::new("verify").about("Execute the NAPE-managed current Verification run.")
}

#[cfg(test)]
mod tests {
    use super::{evidence, package, start, verify};
    use clap::error::ErrorKind;

    const PURL: &str = "pkg:attestify/acme.example/verification-procedure/release/readiness@1.0.0";
    const DIGEST: &str = "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    /// Requirement validation: exercises one bounded logical path.

    #[test]
    fn package_registry_configuration_is_exactly_one_profile_or_endpoint_success() {
        let missing = package()
            .try_get_matches_from([
                "package",
                "resolve",
                "--package",
                PURL,
                "--manifest-digest",
                DIGEST,
                "--plan-only",
            ])
            .expect_err("test path should reject");
        assert_eq!(missing.kind(), ErrorKind::MissingRequiredArgument);

        package()
            .try_get_matches_from([
                "package",
                "resolve",
                "--package",
                PURL,
                "--manifest-digest",
                DIGEST,
                "--registry-endpoint",
                "http://localhost:5001",
                "--plan-only",
            ])
            .expect("single registry endpoint");

        let conflicting = package()
            .try_get_matches_from([
                "package",
                "publish",
                "--local-package",
                "package-output",
                "--registry-profile",
                "registry-map.yaml",
                "--registry-endpoint",
                "http://localhost:5001",
            ])
            .expect_err("test path should reject");
        assert_eq!(conflicting.kind(), ErrorKind::ArgumentConflict);
    }

    /// Requirement validation: exercises one bounded logical path.

    #[test]
    fn verify_registry_configuration_is_required_only_for_oci_source_success() {
        start()
            .try_get_matches_from([
                "start",
                "--local-package",
                "package-output",
                "--subject-file",
                "subject.json",
            ])
            .expect("local verification");

        start()
            .try_get_matches_from([
                "start",
                "--package",
                PURL,
                "--manifest-digest",
                DIGEST,
                "--registry-endpoint",
                "http://localhost:5001",
                "--subject-file",
                "subject.json",
            ])
            .expect("OCI verification with one endpoint");

        let missing = start()
            .try_get_matches_from([
                "start",
                "--package",
                PURL,
                "--manifest-digest",
                DIGEST,
                "--subject-file",
                "subject.json",
            ])
            .expect_err("test path should reject");
        assert_eq!(missing.kind(), ErrorKind::MissingRequiredArgument);
    }

    /// Requirement validation: exercises one bounded logical path.

    #[test]
    fn staged_evidence_and_verify_have_the_closed_shapes_success() {
        evidence()
            .try_get_matches_from([
                "evidence",
                "--action",
                "release-readiness.database-connection",
                "--file",
                "exports/config.json",
            ])
            .expect("one-file Evidence form");
        verify()
            .try_get_matches_from(["verify"])
            .expect("argument-free Verify");
        assert!(verify()
            .try_get_matches_from(["verify", "--output", "result"])
            .is_err());
    }
}
