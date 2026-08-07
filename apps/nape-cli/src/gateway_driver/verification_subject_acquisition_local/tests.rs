use std::fs;

use kernel_oss::gateway::Gateway;
use nape_domain::{diagnostic::NapeOutcome, value::subject::VerificationSubjectHandle};

use super::LocalVerificationSubjectAcquisitionDriver;

/// Requirement validation: exercises one bounded logical path.

#[test]
fn closed_subject_is_admitted_without_interpreting_its_external_arn_success() {
    let path = std::env::temp_dir().join(format!("nape-subject-{}.yaml", std::process::id()));
    fs::write(
        &path,
        b"arn: risk-source:01KWJK2M4W6AX3XJ7C0Q8N5R2T\nlabel: Orders\n",
    )
    .expect("subject");
    let outcome = Gateway::execute(
        &LocalVerificationSubjectAcquisitionDriver,
        VerificationSubjectHandle::try_new(path.to_string_lossy()).expect("handle"),
    )
    .expect("gateway");
    let NapeOutcome::Completed(subject) = outcome else {
        panic!("subject rejected");
    };
    assert_eq!(subject.label(), Some("Orders"));
    fs::remove_file(path).expect("cleanup");
}
