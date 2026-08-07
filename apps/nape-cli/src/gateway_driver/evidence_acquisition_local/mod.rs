//! Stable one-file evidence acquisition driver.

#[cfg(test)]
mod tests;

use kernel_oss::{error::Error, gateway::Gateway};
use nape_domain::{
    diagnostic::{NapeDiagnostic, NapeOutcome},
    gateway::evidence_acquisition::{
        AdmittedEvidencePayload, EvidenceAcquisitionGW, EvidenceAcquisitionRequest,
    },
    value::package::ManifestDigest,
};
use sha2::{Digest, Sha256};

use super::local_file::read_regular_stable;

/// Acquires one immutable local evidence payload.
pub struct LocalEvidenceAcquisitionDriver;

impl Gateway for LocalEvidenceAcquisitionDriver {
    type Request = EvidenceAcquisitionRequest;
    type Response = NapeOutcome<AdmittedEvidencePayload>;

    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
        let path = std::path::Path::new(request.source.value());
        let bytes = match read_regular_stable(path, request.requirement.maximum_bytes()) {
            Ok(bytes) => bytes,
            Err(_) => {
                return Ok(NapeOutcome::Rejected(NapeDiagnostic::try_new(
                    "evidence_acquisition_failed",
                    "evidence-acquisition-failed-v1",
                    "evidence-acquisition",
                    "evidence source is unavailable, unstable, or exceeds its effective maximum",
                )?));
            }
        };
        let digest = format!("sha256:{}", hex::encode(Sha256::digest(&bytes)));
        Ok(NapeOutcome::Completed(AdmittedEvidencePayload {
            digest: ManifestDigest::try_new(digest)?,
            bytes,
        }))
    }
}

impl EvidenceAcquisitionGW for LocalEvidenceAcquisitionDriver {}
