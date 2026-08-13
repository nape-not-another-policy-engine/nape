use std::{collections::BTreeMap, fs, path::PathBuf};

use kernel_oss::gateway::Gateway;
use nape_domain::{
    diagnostic::NapeOutcome,
    gateway::{
        evidence_acquisition::AdmittedEvidencePayload,
        verification_evidence_commit::VerificationEvidenceCommitRequest,
        verification_outcome_commit::VerificationOutcomeCommitRequest,
        verification_start_commit::VerificationStartCommitRequest,
    },
    value::{
        controlled_value::ControlledValue,
        current_verification::{
            CurrentVerification, VerificationEvidenceState, VerificationInvocationId,
            VerificationStartState,
        },
        effective_graph::CanonicalActionSelector,
        evidence::{EvidenceFileName, EvidenceRequirement},
        invocation_metadata::InvocationMetadata,
        package::ManifestDigest,
        subject::VerificationSubject,
    },
};
use sha2::Digest;

use super::LocalVerificationStartCommitDriver;
use crate::gateway_driver::{
    current_run_state::CurrentRunStateStore,
    package_support::{project_closure, VerifiedPackageStore},
    verification_evidence_commit_local::LocalVerificationEvidenceCommitDriver,
    verification_outcome_commit_local::LocalVerificationOutcomeCommitDriver,
};

/// Requirement validation: exercises one bounded logical path.

#[test]
fn start_freezes_reopenable_package_custody_success() {
    let fixture = fixture("custody");
    let outcome = Gateway::execute(
        &LocalVerificationStartCommitDriver::new(fixture.state.clone()),
        VerificationStartCommitRequest {
            verification: fixture.current,
        },
    )
    .expect("Start driver");
    let NapeOutcome::Completed(_) = outcome else {
        panic!("Start rejected");
    };
    let reopened = fixture.state.load().expect("reopen current state");
    assert_eq!(
        reopened
            .package_closure()
            .root()
            .identity()
            .manifest_digest(),
        fixture.domain_closure.root().identity().manifest_digest()
    );
    fs::remove_dir_all(fixture.root).expect("cleanup");
}

/// Requirement validation: a later command rejects tampered frozen package custody.
#[test]
fn start_frozen_package_custody_rejects_tampering_error() {
    let fixture = fixture("custody-tamper");
    let outcome = Gateway::execute(
        &LocalVerificationStartCommitDriver::new(fixture.state.clone()),
        VerificationStartCommitRequest {
            verification: fixture.current,
        },
    )
    .expect("Start driver");
    let NapeOutcome::Completed(observation) = outcome else {
        panic!("Start rejected");
    };
    let digest = fixture
        .raw
        .identity
        .manifest_digest
        .strip_prefix("sha256:")
        .expect("digest");
    fs::write(
        PathBuf::from(observation.current_run.value()).join(format!(
            "package-custody/packages/sha256-{digest}/manifest.json"
        )),
        b"{}",
    )
    .expect("tamper");
    assert!(fixture.state.load().is_err());
    fs::remove_dir_all(fixture.root).expect("cleanup");
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn evidence_and_result_commits_preserve_the_current_package_boundary_success() {
    let fixture = fixture("commit");
    let outcome = Gateway::execute(
        &LocalVerificationStartCommitDriver::new(fixture.state.clone()),
        VerificationStartCommitRequest {
            verification: fixture.current,
        },
    )
    .expect("Start driver");
    let NapeOutcome::Completed(started) = outcome else {
        panic!("Start rejected");
    };
    let current = fixture.state.load().expect("current");
    let requirement = current.evidence_requirements()[0].clone();
    let bytes = br#"{"endpoint":"postgresql://orders"}"#.to_vec();
    let digest = ManifestDigest::try_new(format!(
        "sha256:{}",
        hex::encode(sha2::Sha256::digest(&bytes))
    ))
    .expect("digest");
    let outcome = Gateway::execute(
        &LocalVerificationEvidenceCommitDriver::new(fixture.state.clone()),
        VerificationEvidenceCommitRequest {
            current,
            requirement,
            payload: AdmittedEvidencePayload {
                digest: digest.clone(),
                bytes: bytes.clone(),
            },
        },
    )
    .expect("Evidence driver");
    assert!(matches!(outcome, NapeOutcome::Completed(_)));
    let current = fixture.state.load().expect("current with Evidence");
    assert_eq!(current.evidence_associations().len(), 1);

    let report = object([("kind", string("VerificationReport"))]);
    let relationship = object([("kind", string("VerificationEvidenceSetRelationship"))]);
    let output = current.result_output().expect("result output").clone();
    let outcome = Gateway::execute(
        &LocalVerificationOutcomeCommitDriver,
        VerificationOutcomeCommitRequest {
            report,
            evidence_set_relationship: relationship,
            evidence_payloads: BTreeMap::from([(digest, bytes)]),
            output,
        },
    )
    .expect("Outcome driver");
    let NapeOutcome::Completed(committed) = outcome else {
        panic!("Outcome rejected");
    };
    assert_eq!(committed.file_count, 3);
    assert!(PathBuf::from(started.current_run.value())
        .join("verification-result/verification-report.json")
        .is_file());
    fs::remove_dir_all(fixture.root).expect("cleanup");
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn later_command_reopens_the_current_run_from_a_different_working_directory_success() {
    let fixture = fixture("other-working-directory");
    let outcome = Gateway::execute(
        &LocalVerificationStartCommitDriver::new(fixture.state.clone()),
        VerificationStartCommitRequest {
            verification: fixture.current,
        },
    )
    .expect("Start driver");
    assert!(matches!(outcome, NapeOutcome::Completed(_)));
    let other = fixture.root.join("unrelated-command-directory");
    fs::create_dir(&other).expect("other working directory");
    let state = CurrentRunStateStore::new(
        fixture.root.join("home"),
        other,
        VerifiedPackageStore::default(),
    );
    assert!(state.load().is_ok());
    fs::remove_dir_all(fixture.root).expect("cleanup");
}

struct Fixture {
    root: PathBuf,
    state: CurrentRunStateStore,
    current: CurrentVerification,
    domain_closure: nape_domain::value::package::VerifiedPackageClosure,
    raw: attestify_oci_oss::VerifiedPackage,
}

fn fixture(label: &str) -> Fixture {
    let root = std::env::temp_dir().join(format!(
        "nape-c3-{label}-{}-{}",
        std::process::id(),
        super::NEXT_START.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    ));
    let _ = fs::remove_dir_all(&root);
    let source = root.join("source");
    let build = root.join("build");
    let work = root.join("work");
    let home = root.join("home");
    fs::create_dir_all(source.join("activity/release-readiness/database-connection"))
        .expect("source");
    fs::create_dir_all(&work).expect("work");
    fs::create_dir_all(&home).expect("home");
    fs::write(
        source.join("verification-procedure.yaml"),
        br#"apiVersion: 2.0.0
kind: VerificationProcedure
spec:
  name: database-endpoint
  label: Database Endpoint
  activity:
    - name: release-readiness
      label: Release Readiness
      action:
        - name: database-connection
          label: Database Connection
          claim: The endpoint is configured.
          evidence:
            name: application-configuration
            file: application-configuration.json
          test:
            name: database-connection
            file: database-connection.py
"#,
    )
    .expect("root definition");
    fs::write(
        source.join("activity/release-readiness/database-connection/database-connection.py"),
        b"def evaluate(evidence, evaluations, metadata): return {'conclusion': 'true', 'facts': None, 'reason': 'ok'}\n",
    )
    .expect("Test");
    let package =
        "pkg:attestify/acme.example/verification-procedure/database/database-endpoint@1.0.0";
    let profile =
        crate::gateway_driver::definition_package_profile_attestify_oci::definition_package_profile()
            .expect("profile");
    attestify_oci_oss::build_local_package(
        profile.clone(),
        "VerificationProcedure",
        &source,
        package,
        &build,
    )
    .expect("build");
    let raw = attestify_oci_oss::admit_local_package(profile, &build, &root.join("admission"))
        .expect("admit build");
    let domain_closure = project_closure(&raw, &[]).expect("Domain projection");
    let requirement = EvidenceRequirement::try_new(
        CanonicalActionSelector::try_new("release-readiness.database-connection")
            .expect("selector"),
        "application-configuration",
        EvidenceFileName::try_new("application-configuration.json").expect("file"),
        256 * 1_024 * 1_024,
    )
    .expect("requirement");
    let current = CurrentVerification::new(
        VerificationStartState::new(
            VerificationInvocationId::try_new("01KWJK2J5W0VYF6V72JYF6DTRQ").expect("invocation"),
            VerificationSubject::try_new("risk-source:01KWJK2M4W6AX3XJ7C0Q8N5R2T", None, None)
                .expect("subject"),
            InvocationMetadata::from_entries(Vec::new()),
            domain_closure.clone(),
            1,
        ),
        VerificationEvidenceState::new(vec![requirement], Vec::new(), BTreeMap::new()),
    );
    let packages = VerifiedPackageStore::default();
    packages.insert(raw.clone()).expect("store package");
    let state = CurrentRunStateStore::new(&home, &work, packages);
    Fixture {
        root,
        state,
        current,
        domain_closure,
        raw,
    }
}

fn string(value: &str) -> ControlledValue {
    ControlledValue::String(value.to_string())
}

fn object<const N: usize>(values: [(&str, ControlledValue); N]) -> ControlledValue {
    ControlledValue::Object(
        values
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect(),
    )
}
