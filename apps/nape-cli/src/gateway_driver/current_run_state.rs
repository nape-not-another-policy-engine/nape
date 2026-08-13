//! Versioned complete current-run state codec and atomic state-file owner.

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use attestify_oci_oss::custody::{
    reopen_verified_closure, CustodiedPackage, CustodiedPackageClosure, PackageCustodyToken,
};
use kernel_oss::error::{Error, Kind};
use nape_domain::value::{
    current_verification::{
        CurrentVerification, VerificationAcquisition, VerificationEvidenceState,
        VerificationInvocationId, VerificationStartState,
    },
    effective_graph::CanonicalActionSelector,
    evidence::{EvidenceAssociation, EvidenceFileName, EvidenceRequirement},
    external_resource_handle::ExternalResourceHandle,
    invocation_metadata::{InvocationMetadata, InvocationMetadataEntry},
    package::{ManifestDigest, PackageReleasePurl},
    subject::VerificationSubject,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{
    definition_package_profile_attestify_oci::definition_package_profile,
    package_support::{project_closure, VerifiedPackageStore},
};

const STATE_VERSION: &str = "attestify.nape.current-verification/v2";
const STATE_MAX_BYTES: u64 = 256 * 1_024 * 1_024;
static NEXT_STATE: AtomicU64 = AtomicU64::new(1);

/// Explicit filesystem locations owned by the NAPE Application layer.
#[derive(Clone)]
pub struct CurrentRunStateStore {
    state_file: PathBuf,
    working_directory: PathBuf,
    packages: VerifiedPackageStore,
}

impl CurrentRunStateStore {
    /// Creates a store from explicit home and command working directories.
    pub fn new(
        home: impl Into<PathBuf>,
        working_directory: impl Into<PathBuf>,
        packages: VerifiedPackageStore,
    ) -> Self {
        Self {
            state_file: home.into().join("nape/.nape_cli_config"),
            working_directory: working_directory.into(),
            packages,
        }
    }

    pub(crate) fn state_file(&self) -> &Path {
        &self.state_file
    }
    pub(crate) fn working_directory(&self) -> &Path {
        &self.working_directory
    }

    pub(crate) fn packages(&self) -> &VerifiedPackageStore {
        &self.packages
    }

    pub(crate) fn load(&self) -> Result<CurrentVerification, Error> {
        let bytes = super::local_file::read_regular_stable(&self.state_file, STATE_MAX_BYTES)?;
        let wire: CurrentVerificationWire = serde_yaml::from_slice(&bytes).map_err(|_| {
            Error::for_user(Kind::InvalidInput, "current Verification state is invalid")
        })?;
        let run = Path::new(&wire.run);
        if !run.is_absolute() {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "current Verification run location is not absolute",
            ));
        }
        wire.into_domain(&self.packages)
    }

    pub(crate) fn commit(&self, current: &CurrentVerification) -> Result<(), Error> {
        let parent = self.state_file.parent().ok_or_else(|| {
            Error::for_system(Kind::ProcessingFailure, "current state file has no parent")
        })?;
        std::fs::create_dir_all(parent).map_err(|_| {
            Error::for_system(Kind::GatewayError, "current state directory is unavailable")
        })?;
        let wire = CurrentVerificationWire::from_domain(current)?;
        let bytes = serde_yaml::to_string(&wire)
            .map_err(|_| {
                Error::for_system(
                    Kind::ProcessingFailure,
                    "current state cannot be serialized",
                )
            })?
            .into_bytes();
        let temporary = parent.join(format!(
            ".nape-current-{}-{}",
            std::process::id(),
            NEXT_STATE.fetch_add(1, Ordering::Relaxed)
        ));
        write_new(&temporary, &bytes)?;
        std::fs::rename(&temporary, &self.state_file).map_err(|_| {
            let _ = std::fs::remove_file(&temporary);
            Error::for_system(
                Kind::GatewayError,
                "current state atomic replacement failed",
            )
        })
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CurrentVerificationWire {
    version: String,
    invocation_id: String,
    subject: SubjectWire,
    metadata: BTreeMap<String, String>,
    package_closure: PackageClosureWire,
    evidence_requirement: Vec<EvidenceRequirementWire>,
    evidence_association: Vec<EvidenceAssociationWire>,
    run: String,
    result_output: String,
    utc_start_milliseconds: u64,
    acquisition: String,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SubjectWire {
    arn: String,
    label: Option<String>,
    description: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PackageClosureWire {
    root: PackageWire,
    dependency: Vec<PackageWire>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PackageWire {
    package: String,
    manifest_digest: String,
    custody_token: String,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct EvidenceRequirementWire {
    action: String,
    name: String,
    file: String,
    maximum_bytes: u64,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct EvidenceAssociationWire {
    action: String,
    digest: String,
    byte_count: u64,
}

impl CurrentVerificationWire {
    fn from_domain(value: &CurrentVerification) -> Result<Self, Error> {
        Ok(Self {
            version: STATE_VERSION.to_string(),
            invocation_id: value.invocation_id().value().to_string(),
            subject: SubjectWire {
                arn: value.subject().arn().to_string(),
                label: value.subject().label().map(str::to_string),
                description: value.subject().description().map(str::to_string),
            },
            metadata: value.metadata().values().clone(),
            package_closure: PackageClosureWire {
                root: PackageWire::from_domain(value.package_closure().root())?,
                dependency: value
                    .package_closure()
                    .dependencies()
                    .iter()
                    .map(PackageWire::from_domain)
                    .collect::<Result<Vec<_>, _>>()?,
            },
            evidence_requirement: value
                .evidence_requirements()
                .iter()
                .map(|requirement| EvidenceRequirementWire {
                    action: requirement.action().value().to_string(),
                    name: requirement.name().to_string(),
                    file: requirement.file().value().to_string(),
                    maximum_bytes: requirement.maximum_bytes(),
                })
                .collect(),
            evidence_association: value
                .evidence_associations()
                .iter()
                .map(|association| EvidenceAssociationWire {
                    action: association.action.value().to_string(),
                    digest: association.digest.value().to_string(),
                    byte_count: association.byte_count,
                })
                .collect(),
            run: value
                .run()
                .ok_or_else(|| {
                    Error::for_system(
                        Kind::ProcessingFailure,
                        "current state omitted its managed run",
                    )
                })?
                .value()
                .to_string(),
            result_output: value
                .result_output()
                .ok_or_else(|| {
                    Error::for_system(
                        Kind::ProcessingFailure,
                        "current state omitted its result output",
                    )
                })?
                .value()
                .to_string(),
            utc_start_milliseconds: value.utc_start_milliseconds(),
            acquisition: value.acquisition().source().to_string(),
        })
    }

    fn into_domain(self, packages: &VerifiedPackageStore) -> Result<CurrentVerification, Error> {
        if self.version != STATE_VERSION {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "current Verification state version is unsupported",
            ));
        }
        let requirements = self
            .evidence_requirement
            .into_iter()
            .map(|value| {
                EvidenceRequirement::try_new(
                    CanonicalActionSelector::try_new(value.action)?,
                    value.name,
                    EvidenceFileName::try_new(value.file)?,
                    value.maximum_bytes,
                )
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let associations = self
            .evidence_association
            .into_iter()
            .map(|value| {
                Ok(EvidenceAssociation {
                    action: CanonicalActionSelector::try_new(value.action)?,
                    digest: ManifestDigest::try_new(value.digest)?,
                    byte_count: value.byte_count,
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let mut payloads = BTreeMap::new();
        for association in &associations {
            let requirement = requirements
                .iter()
                .find(|requirement| requirement.action() == &association.action)
                .ok_or_else(|| {
                    Error::for_user(
                        Kind::InvalidInput,
                        "current Evidence association has no requirement",
                    )
                })?;
            let (activity, action) =
                association.action.value().split_once('.').ok_or_else(|| {
                    Error::for_user(
                        Kind::InvalidInput,
                        "current Evidence association selector is invalid",
                    )
                })?;
            let path = Path::new(&self.run)
                .join("evidence")
                .join(activity)
                .join(action)
                .join(requirement.file().value());
            let bytes = super::local_file::read_regular_stable(&path, requirement.maximum_bytes())?;
            if bytes.len() as u64 != association.byte_count {
                return Err(Error::for_user(
                    Kind::InvalidInput,
                    "current Evidence association byte count is invalid",
                ));
            }
            if format!("sha256:{}", hex::encode(Sha256::digest(&bytes)))
                != association.digest.value()
            {
                return Err(Error::for_user(
                    Kind::InvalidInput,
                    "current Evidence payload digest is invalid",
                ));
            }
            if payloads
                .insert(association.digest.clone(), bytes.clone())
                .is_some_and(|prior| prior != bytes)
            {
                return Err(Error::for_user(
                    Kind::InvalidInput,
                    "one Evidence digest maps to conflicting payload bytes",
                ));
            }
        }
        let custody = CustodiedPackageClosure {
            root: self.package_closure.root.into_custody()?,
            dependencies: self
                .package_closure
                .dependency
                .into_iter()
                .map(PackageWire::into_custody)
                .collect::<Result<Vec<_>, Error>>()?,
        };
        let custody_root = Path::new(&self.run).join("package-custody");
        let profile = definition_package_profile()?;
        let raw = reopen_verified_closure(&profile, &custody_root, &custody)
            .map_err(package_state_error)?;
        packages.insert_closure(&raw)?;
        let package_closure = project_closure(&raw.root, &raw.dependencies)?;
        let run = ExternalResourceHandle::try_new(self.run)?;
        let output = ExternalResourceHandle::try_new(self.result_output)?;
        let metadata = self
            .metadata
            .into_iter()
            .map(|(key, value)| InvocationMetadataEntry::try_new(key, value))
            .collect::<Result<Vec<_>, _>>()?;
        let graph = nape_domain::service::effective_graph::resolve_effective_verification_graph(
            &package_closure,
        )?;
        let acquisition = match self.acquisition.as_str() {
            "local-build" => VerificationAcquisition::LocalBuild,
            "oci-pull" => VerificationAcquisition::OciPull,
            _ => {
                return Err(Error::for_user(
                    Kind::InvalidInput,
                    "current Verification acquisition is invalid",
                ))
            }
        };
        Ok(CurrentVerification::new(
            VerificationStartState::new(
                VerificationInvocationId::try_new(self.invocation_id)?,
                VerificationSubject::try_new(
                    self.subject.arn,
                    self.subject.label,
                    self.subject.description,
                )?,
                InvocationMetadata::from_entries(metadata),
                package_closure,
                self.utc_start_milliseconds,
            ),
            VerificationEvidenceState::new(requirements, associations, payloads),
        )
        .with_resolution(graph, acquisition)
        .with_managed_locations(run, output))
    }
}

impl PackageWire {
    fn from_domain(value: &nape_domain::value::package::VerifiedPackage) -> Result<Self, Error> {
        let identity = attestify_oci_oss::PackageIdentity {
            package: value.identity().purl().value().to_string(),
            manifest_digest: value.identity().manifest_digest().value().to_string(),
        };
        let token = PackageCustodyToken::for_identity(&identity).map_err(package_state_error)?;
        Ok(Self {
            package: identity.package,
            manifest_digest: identity.manifest_digest,
            custody_token: token.value().to_string(),
        })
    }

    fn into_custody(self) -> Result<CustodiedPackage, Error> {
        let identity = attestify_oci_oss::PackageIdentity {
            package: PackageReleasePurl::try_new(self.package)?
                .value()
                .to_string(),
            manifest_digest: ManifestDigest::try_new(self.manifest_digest)?
                .value()
                .to_string(),
        };
        Ok(CustodiedPackage {
            identity,
            token: PackageCustodyToken::try_new(self.custody_token).map_err(package_state_error)?,
        })
    }
}

fn package_state_error(_: attestify_oci_oss::PackageError) -> Error {
    Error::for_user(
        Kind::InvalidInput,
        "current package reconstruction failed integrity verification",
    )
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), Error> {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|_| {
            Error::for_system(
                Kind::GatewayError,
                "private state staging file cannot be created",
            )
        })?;
    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|_| {
            Error::for_system(
                Kind::GatewayError,
                "private state staging file cannot be committed",
            )
        })
}
