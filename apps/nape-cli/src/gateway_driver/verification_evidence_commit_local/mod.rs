//! Atomic one-file evidence commitment into the current NAPE run.

#[cfg(test)]
mod tests;

use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use kernel_oss::{
    error::{Error, Kind},
    gateway::Gateway,
};
use nape_domain::{
    diagnostic::NapeOutcome,
    gateway::verification_evidence_commit::{
        EvidenceCommitDisposition, VerificationEvidenceCommitGW,
        VerificationEvidenceCommitObservation, VerificationEvidenceCommitRequest,
    },
    value::evidence::EvidenceAssociation,
};

use super::{current_run_state::CurrentRunStateStore, local_file};

static NEXT_EVIDENCE: AtomicU64 = AtomicU64::new(1);

/// Commits one admitted immutable payload and its occurrence association.
pub struct LocalVerificationEvidenceCommitDriver {
    state: CurrentRunStateStore,
}

impl LocalVerificationEvidenceCommitDriver {
    /// Creates the driver around explicit NAPE state ownership.
    pub fn new(state: CurrentRunStateStore) -> Self {
        Self { state }
    }
}

impl Gateway for LocalVerificationEvidenceCommitDriver {
    type Request = VerificationEvidenceCommitRequest;
    type Response = NapeOutcome<VerificationEvidenceCommitObservation>;

    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
        let prior = request
            .current
            .evidence_associations()
            .iter()
            .find(|value| value.action == *request.requirement.action())
            .cloned();
        let disposition = match prior.as_ref() {
            None => EvidenceCommitDisposition::Added,
            Some(value) if value.digest == request.payload.digest => {
                EvidenceCommitDisposition::Unchanged
            }
            Some(_) => EvidenceCommitDisposition::Replaced,
        };

        let mut associations = request.current.evidence_associations().to_vec();
        associations.retain(|value| value.action != *request.requirement.action());
        associations.push(EvidenceAssociation {
            action: request.requirement.action().clone(),
            digest: request.payload.digest.clone(),
            byte_count: request.payload.bytes.len() as u64,
        });
        associations.sort_by(|left, right| left.action.cmp(&right.action));

        let mut payloads = request.current.evidence_payloads().clone();
        payloads.insert(
            request.payload.digest.clone(),
            request.payload.bytes.clone(),
        );
        if let Some(prior) = &prior {
            if prior.digest != request.payload.digest
                && !associations
                    .iter()
                    .any(|value| value.digest == prior.digest)
            {
                payloads.remove(&prior.digest);
            }
        }

        let run = request.current.run().ok_or_else(|| {
            Error::for_system(Kind::ProcessingFailure, "current run location is absent")
        })?;
        let canonical = canonical_evidence_path(
            Path::new(run.value()),
            request.requirement.action().value(),
            request.requirement.file().value(),
        )?;
        let parent = canonical.parent().ok_or_else(|| {
            Error::for_system(Kind::ProcessingFailure, "Evidence path has no parent")
        })?;
        ensure_confined_directories(Path::new(run.value()), parent)?;

        if disposition == EvidenceCommitDisposition::Unchanged {
            let current_bytes =
                local_file::read_regular_stable(&canonical, request.requirement.maximum_bytes())?;
            if current_bytes != request.payload.bytes {
                return Err(Error::for_user(
                    Kind::InvalidInput,
                    "current Evidence file does not match its frozen digest",
                ));
            }
            return Ok(NapeOutcome::Completed(
                VerificationEvidenceCommitObservation {
                    disposition,
                    unique_payload_count: payloads.len() as u64,
                },
            ));
        }

        let serial = NEXT_EVIDENCE.fetch_add(1, Ordering::SeqCst);
        let staging = parent.join(format!(".nape-evidence-{}-{serial}", std::process::id()));
        let backup = parent.join(format!(
            ".nape-evidence-backup-{}-{serial}",
            std::process::id()
        ));
        write_new(&staging, &request.payload.bytes)?;
        let had_prior_file = path_exists(&canonical)?;
        if had_prior_file {
            fs::rename(&canonical, &backup).map_err(gateway_error)?;
        }
        if let Err(error) = fs::rename(&staging, &canonical).map_err(gateway_error) {
            if had_prior_file {
                let _ = fs::rename(&backup, &canonical);
            }
            let _ = fs::remove_file(&staging);
            return Err(error);
        }
        sync_directory(parent)?;

        let updated = request
            .current
            .with_evidence_state(associations, payloads.clone());
        if let Err(error) = self.state.commit(&updated) {
            let _ = fs::remove_file(&canonical);
            if had_prior_file {
                let _ = fs::rename(&backup, &canonical);
            }
            return Err(error);
        }
        if had_prior_file {
            fs::remove_file(&backup).map_err(gateway_error)?;
            sync_directory(parent)?;
        }
        Ok(NapeOutcome::Completed(
            VerificationEvidenceCommitObservation {
                disposition,
                unique_payload_count: payloads.len() as u64,
            },
        ))
    }
}

impl VerificationEvidenceCommitGW for LocalVerificationEvidenceCommitDriver {}

fn canonical_evidence_path(run: &Path, selector: &str, file: &str) -> Result<PathBuf, Error> {
    let (activity, action) = selector.split_once('.').ok_or_else(|| {
        Error::for_system(Kind::ProcessingFailure, "Action selector is not canonical")
    })?;
    Ok(run.join("evidence").join(activity).join(action).join(file))
}

fn ensure_confined_directories(run: &Path, target: &Path) -> Result<(), Error> {
    if !target.starts_with(run) {
        return Err(Error::for_system(
            Kind::ProcessingFailure,
            "Evidence path escaped the current run",
        ));
    }
    let relative = target
        .strip_prefix(run)
        .map_err(|_| Error::for_system(Kind::ProcessingFailure, "Evidence path is not confined"))?;
    let mut current = run.to_path_buf();
    for component in relative.components() {
        current.push(component);
        if path_exists(&current)? {
            let metadata = fs::symlink_metadata(&current).map_err(gateway_error)?;
            if metadata.file_type().is_symlink() || !metadata.is_dir() {
                return Err(Error::for_user(
                    Kind::InvalidInput,
                    "Evidence directory contains a non-directory entry",
                ));
            }
        } else {
            fs::create_dir(&current).map_err(gateway_error)?;
        }
    }
    Ok(())
}

fn path_exists(path: &Path) -> Result<bool, Error> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(gateway_error(error)),
    }
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), Error> {
    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(gateway_error)?;
    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(gateway_error)
}

fn sync_directory(path: &Path) -> Result<(), Error> {
    fs::File::open(path)
        .and_then(|directory| directory.sync_all())
        .map_err(gateway_error)
}

fn gateway_error(_: std::io::Error) -> Error {
    Error::for_system(
        Kind::GatewayError,
        "Evidence commitment filesystem operation failed",
    )
}
