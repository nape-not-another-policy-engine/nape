//! Atomic NAPE-managed current-run creation and selection.

#[cfg(test)]
mod tests;

use std::{
    fs,
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

use attestify_oci::custody::materialize_verified_closure;
use kernel_oss::{
    error::{Error, Kind},
    gateway::Gateway,
};
use nape_domain::{
    diagnostic::NapeOutcome,
    gateway::verification_start_commit::{
        VerificationStartCommitGW, VerificationStartCommitObservation,
        VerificationStartCommitRequest,
    },
    value::external_resource_handle::ExternalResourceHandle,
};

use super::current_run_state::CurrentRunStateStore;

static NEXT_START: AtomicU64 = AtomicU64::new(1);

/// Commits one prepared Start beneath the command working directory.
pub struct LocalVerificationStartCommitDriver {
    state: CurrentRunStateStore,
}

impl LocalVerificationStartCommitDriver {
    /// Creates the driver around explicit NAPE state ownership.
    pub fn new(state: CurrentRunStateStore) -> Self {
        Self { state }
    }
}

impl Gateway for LocalVerificationStartCommitDriver {
    type Request = VerificationStartCommitRequest;
    type Response = NapeOutcome<VerificationStartCommitObservation>;

    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
        let subject_directory = self
            .state
            .working_directory()
            .join(encode_subject_arn(request.verification.subject().arn()));
        ensure_directory(&subject_directory)?;
        let final_run = subject_directory.join(request.verification.invocation_id().value());
        if path_exists(&final_run)? {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "Verification run already exists",
            ));
        }
        let staging = subject_directory.join(format!(
            ".nape-start-{}-{}",
            std::process::id(),
            NEXT_START.fetch_add(1, Ordering::SeqCst)
        ));
        fs::create_dir(&staging).map_err(gateway_error)?;
        set_private_directory(&staging)?;

        let result = (|| {
            let raw = self
                .state
                .packages()
                .get_closure(request.verification.package_closure())?;
            materialize_verified_closure(&raw, &staging.join("package-custody"))
                .map_err(package_error)?;
            fs::create_dir(staging.join("evidence")).map_err(gateway_error)?;
            sync_directory(&staging)?;
            fs::rename(&staging, &final_run).map_err(gateway_error)?;
            sync_directory(&subject_directory)?;

            let output = final_run.join("verification-result");
            let run_handle = external_handle(&final_run)?;
            let output_handle = external_handle(&output)?;
            let committed = request
                .verification
                .with_managed_locations(run_handle.clone(), output_handle.clone());
            if let Err(error) = self.state.commit(&committed) {
                let _ = fs::remove_dir_all(&final_run);
                return Err(error);
            }
            Ok(VerificationStartCommitObservation {
                current_run: run_handle,
                state: external_handle(self.state.state_file())?,
                result_output: output_handle,
            })
        })();
        if result.is_err() {
            let _ = fs::remove_dir_all(&staging);
        }
        result.map(NapeOutcome::Completed)
    }
}

impl VerificationStartCommitGW for LocalVerificationStartCommitDriver {}

fn encode_subject_arn(value: &str) -> String {
    value
        .bytes()
        .map(|byte| match byte {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' => byte as char,
            _ => '_',
        })
        .collect()
}

fn ensure_directory(path: &Path) -> Result<(), Error> {
    if path_exists(path)? {
        let metadata = fs::symlink_metadata(path).map_err(gateway_error)?;
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "Verification run parent is not a regular directory",
            ));
        }
        return Ok(());
    }
    fs::create_dir(path).map_err(gateway_error)?;
    set_private_directory(path)
}

fn path_exists(path: &Path) -> Result<bool, Error> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(gateway_error(error)),
    }
}

fn external_handle(path: &Path) -> Result<ExternalResourceHandle, Error> {
    ExternalResourceHandle::try_new(
        path.to_str().ok_or_else(|| {
            Error::for_system(Kind::GatewayError, "NAPE-managed path is not UTF-8")
        })?,
    )
}

fn package_error(_: attestify_oci::PackageError) -> Error {
    Error::for_user(
        Kind::InvalidInput,
        "verified package closure could not be placed in run custody",
    )
}

fn gateway_error(_: std::io::Error) -> Error {
    Error::for_system(
        Kind::GatewayError,
        "Verification run filesystem operation failed",
    )
}

fn sync_directory(path: &Path) -> Result<(), Error> {
    fs::File::open(path)
        .and_then(|directory| directory.sync_all())
        .map_err(gateway_error)
}

fn set_private_directory(path: &Path) -> Result<(), Error> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).map_err(gateway_error)?;
    }
    Ok(())
}
