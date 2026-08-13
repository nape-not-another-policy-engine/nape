//! Atomic NAPE-owned Verification result-set commitment.

#[cfg(test)]
mod tests;

use std::{
    collections::BTreeSet,
    fs,
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

use kernel_oss::{
    error::{Error, Kind},
    gateway::Gateway,
};
use nape_domain::{
    diagnostic::{NapeDiagnostic, NapeOutcome},
    gateway::verification_outcome_commit::{
        VerificationOutcomeCommitGW, VerificationOutcomeCommitObservation,
        VerificationOutcomeCommitRequest,
    },
};
use sha2::{Digest, Sha256};

use super::controlled_document_projection::to_json;

static NEXT_OUTCOME: AtomicU64 = AtomicU64::new(1);

/// Commits a complete Report, relationship, and digest-addressed Evidence set.
pub struct LocalVerificationOutcomeCommitDriver;

impl Gateway for LocalVerificationOutcomeCommitDriver {
    type Request = VerificationOutcomeCommitRequest;
    type Response = NapeOutcome<VerificationOutcomeCommitObservation>;

    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
        let output = std::path::PathBuf::from(request.output.value());
        if path_exists(&output)? {
            return Ok(NapeOutcome::Rejected(diagnostic(
                "Verification output already exists",
            )?));
        }
        let parent = output.parent().ok_or_else(|| {
            Error::for_system(Kind::ProcessingFailure, "Verification output has no parent")
        })?;
        let parent_metadata = fs::symlink_metadata(parent).map_err(gateway_error)?;
        if parent_metadata.file_type().is_symlink() || !parent_metadata.is_dir() {
            return Ok(NapeOutcome::Rejected(diagnostic(
                "Verification output parent is not a regular directory",
            )?));
        }
        let staging = parent.join(format!(
            ".nape-result-{}-{}",
            std::process::id(),
            NEXT_OUTCOME.fetch_add(1, Ordering::SeqCst)
        ));
        fs::create_dir(&staging).map_err(gateway_error)?;

        let operation = (|| {
            let report = to_json(&request.report)?;
            let relationship = to_json(&request.evidence_set_relationship)?;
            let report_bytes = json_bytes(&report)?;
            let relationship_bytes = json_bytes(&relationship)?;
            write_new(&staging.join("verification-report.json"), &report_bytes)?;
            write_new(
                &staging.join("evidence-set-relationship.json"),
                &relationship_bytes,
            )?;
            let evidence_root = staging.join("evidence/sha256");
            fs::create_dir_all(&evidence_root).map_err(gateway_error)?;
            for (digest, bytes) in &request.evidence_payloads {
                let hex_digest = digest.value().strip_prefix("sha256:").ok_or_else(|| {
                    Error::for_system(Kind::ProcessingFailure, "Evidence digest is invalid")
                })?;
                let actual = hex::encode(Sha256::digest(bytes));
                if actual != hex_digest {
                    return rejected("Evidence bytes do not match their digest");
                }
                write_new(&evidence_root.join(hex_digest), bytes)?;
            }
            verify_stage(
                &staging,
                request.evidence_payloads.keys().map(|value| value.value()),
            )?;
            sync_tree(&staging)?;
            fs::rename(&staging, &output).map_err(gateway_error)?;
            sync_directory(parent)?;
            Ok(VerificationOutcomeCommitObservation {
                output: request.output,
                file_count: request.evidence_payloads.len() as u64 + 2,
            })
        })();
        if operation.is_err() {
            let _ = fs::remove_dir_all(&staging);
        }
        operation.map(NapeOutcome::Completed)
    }
}

impl VerificationOutcomeCommitGW for LocalVerificationOutcomeCommitDriver {}

fn json_bytes(value: &serde_json::Value) -> Result<Vec<u8>, Error> {
    let mut bytes = serde_json::to_vec(value).map_err(|_| {
        Error::for_system(
            Kind::ProcessingFailure,
            "Verification output cannot be serialized",
        )
    })?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn verify_stage<'a>(root: &Path, digests: impl Iterator<Item = &'a str>) -> Result<(), Error> {
    let mut expected = BTreeSet::from([
        "evidence".to_string(),
        "evidence/sha256".to_string(),
        "evidence-set-relationship.json".to_string(),
        "verification-report.json".to_string(),
    ]);
    for digest in digests {
        let value = digest.strip_prefix("sha256:").ok_or_else(|| {
            Error::for_system(Kind::ProcessingFailure, "Evidence digest is invalid")
        })?;
        expected.insert(format!("evidence/sha256/{value}"));
    }
    if inventory(root)? != expected {
        return rejected("Verification result staging contains an unexpected entry");
    }
    let _: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join("verification-report.json")).map_err(gateway_error)?,
    )
    .map_err(|_| Error::for_system(Kind::ProcessingFailure, "Report round trip failed"))?;
    let _: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join("evidence-set-relationship.json")).map_err(gateway_error)?,
    )
    .map_err(|_| Error::for_system(Kind::ProcessingFailure, "relationship round trip failed"))?;
    Ok(())
}

fn inventory(root: &Path) -> Result<BTreeSet<String>, Error> {
    fn walk(root: &Path, directory: &Path, values: &mut BTreeSet<String>) -> Result<(), Error> {
        for entry in fs::read_dir(directory).map_err(gateway_error)? {
            let entry = entry.map_err(gateway_error)?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path).map_err(gateway_error)?;
            if metadata.file_type().is_symlink() || (!metadata.is_dir() && !metadata.is_file()) {
                return rejected("Verification result staging contains a non-regular entry");
            }
            let relative = path.strip_prefix(root).map_err(|_| {
                Error::for_system(Kind::ProcessingFailure, "result staging escaped its root")
            })?;
            values.insert(
                relative
                    .to_string_lossy()
                    .replace(std::path::MAIN_SEPARATOR, "/"),
            );
            if metadata.is_dir() {
                walk(root, &path, values)?;
            }
        }
        Ok(())
    }
    let mut values = BTreeSet::new();
    walk(root, root, &mut values)?;
    Ok(values)
}

fn sync_tree(root: &Path) -> Result<(), Error> {
    fn walk(directory: &Path) -> Result<(), Error> {
        for entry in fs::read_dir(directory).map_err(gateway_error)? {
            let path = entry.map_err(gateway_error)?.path();
            if path.is_dir() {
                walk(&path)?;
            } else {
                fs::File::open(&path)
                    .and_then(|file| file.sync_all())
                    .map_err(gateway_error)?;
            }
        }
        sync_directory(directory)
    }
    walk(root)
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

fn rejected<T>(detail: &str) -> Result<T, Error> {
    Err(Error::for_user(Kind::InvalidInput, detail))
}

fn gateway_error(_: std::io::Error) -> Error {
    Error::for_system(
        Kind::GatewayError,
        "Verification result filesystem operation failed",
    )
}

fn diagnostic(detail: &str) -> Result<NapeDiagnostic, Error> {
    NapeDiagnostic::try_new(
        "engine_execution_integrity_failed",
        "engine-execution-integrity-failed-v1",
        "evidence-set-commit",
        detail,
    )
}
