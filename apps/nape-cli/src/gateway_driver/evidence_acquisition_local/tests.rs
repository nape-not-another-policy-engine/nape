use std::fs;

use kernel_oss::gateway::Gateway;
use nape_domain::{
    diagnostic::NapeOutcome,
    gateway::evidence_acquisition::EvidenceAcquisitionRequest,
    value::{
        effective_graph::CanonicalActionSelector,
        evidence::{EvidenceFileName, EvidenceRequirement, EvidenceSourceHandle},
    },
};

use super::LocalEvidenceAcquisitionDriver;

/// Requirement validation: exercises one bounded logical path.

#[test]
fn arbitrary_source_name_is_read_under_the_definition_ceiling_success() {
    let path =
        std::env::temp_dir().join(format!("nape-evidence-source-{}.json", std::process::id()));
    fs::write(&path, b"{\"ready\":true}").expect("evidence");
    let requirement = EvidenceRequirement::try_new(
        CanonicalActionSelector::try_new("release-readiness.database-connection")
            .expect("selector"),
        "application-configuration",
        EvidenceFileName::try_new("application-configuration.json").expect("filename"),
        1_024,
    )
    .expect("requirement");
    let outcome = Gateway::execute(
        &LocalEvidenceAcquisitionDriver,
        EvidenceAcquisitionRequest {
            requirement,
            source: EvidenceSourceHandle::try_new(path.to_string_lossy()).expect("source"),
        },
    )
    .expect("gateway");
    let NapeOutcome::Completed(payload) = outcome else {
        panic!("evidence rejected");
    };
    assert_eq!(payload.bytes, b"{\"ready\":true}");
    fs::remove_file(path).expect("cleanup");
}
