//! Deterministic one-occurrence evidence association admission.

#[cfg(test)]
mod tests;

use kernel_oss::error::{Error, Kind};

use crate::value::{
    effective_graph::CanonicalActionSelector,
    evidence::{EvidenceFileName, EvidenceRequirement},
};

/// Returns the controlled media type registered for a supported evidence
/// filename suffix.
pub fn evidence_media_type(file: &str) -> Option<&'static str> {
    match file.rsplit_once('.')?.1 {
        "json" => Some("application/json"),
        "yaml" | "yml" => Some("application/yaml"),
        "toml" => Some("application/toml"),
        _ => None,
    }
}

/// Selects one exact occurrence requirement and validates an optional V1 filename assertion.
pub fn select_evidence_requirement(
    requirements: &[EvidenceRequirement],
    action: &CanonicalActionSelector,
    asserted_file_name: Option<&EvidenceFileName>,
) -> Result<EvidenceRequirement, Error> {
    let requirement = requirements
        .iter()
        .find(|requirement| requirement.action() == action)
        .cloned()
        .ok_or_else(|| {
            Error::for_user(
                Kind::NotFound,
                "Action occurrence has no evidence requirement in the current run",
            )
        })?;
    if asserted_file_name.is_some_and(|value| value != requirement.file()) {
        return Err(Error::for_user(
            Kind::InvalidInput,
            "asserted evidence filename does not equal the definition-owned filename",
        ));
    }
    Ok(requirement)
}
