//! Application-private exact package-byte custody and Domain projection.

use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use attestify_oci_oss::VerifiedPackage as OciVerifiedPackage;
use kernel_oss::error::{Error, Kind};
use nape_domain::value::{
    definition::DefinitionKind,
    package::{
        ManifestDigest, PackageLock, PackageLockEdge, PackageLockNode, PackageReleaseIdentity,
        PackageReleasePurl, VerifiedPackage as DomainVerifiedPackage, VerifiedPackageClosure,
    },
};
use sha2::{Digest, Sha256};

/// Shared private custody of verified package bytes keyed by exact release identity.
#[derive(Clone, Default)]
pub struct VerifiedPackageStore(Arc<Mutex<BTreeMap<String, OciVerifiedPackage>>>);

impl VerifiedPackageStore {
    pub(crate) fn insert(&self, package: OciVerifiedPackage) -> Result<(), Error> {
        let key = key(&package.identity.package, &package.identity.manifest_digest);
        let mut values = self.0.lock().map_err(|_| {
            Error::for_system(Kind::GatewayError, "verified package store is unavailable")
        })?;
        if values.get(&key).is_some_and(|current| current != &package) {
            return Err(Error::for_system(
                Kind::ProcessingFailure,
                "verified package identity maps to conflicting package bytes",
            ));
        }
        values.insert(key, package);
        Ok(())
    }

    pub(crate) fn get(
        &self,
        identity: &PackageReleaseIdentity,
    ) -> Result<OciVerifiedPackage, Error> {
        self.0
            .lock()
            .map_err(|_| {
                Error::for_system(Kind::GatewayError, "verified package store is unavailable")
            })?
            .get(&key(
                identity.purl().value(),
                identity.manifest_digest().value(),
            ))
            .cloned()
            .ok_or_else(|| {
                Error::for_system(
                    Kind::ProcessingFailure,
                    "verified package bytes are absent from application custody",
                )
            })
    }

    pub(crate) fn get_closure(
        &self,
        closure: &VerifiedPackageClosure,
    ) -> Result<attestify_oci_oss::VerifiedPackageClosure, Error> {
        Ok(attestify_oci_oss::VerifiedPackageClosure {
            root: self.get(closure.root().identity())?,
            dependencies: closure
                .dependencies()
                .iter()
                .map(|package| self.get(package.identity()))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    pub(crate) fn insert_closure(
        &self,
        closure: &attestify_oci_oss::VerifiedPackageClosure,
    ) -> Result<(), Error> {
        self.insert(closure.root.clone())?;
        for package in &closure.dependencies {
            self.insert(package.clone())?;
        }
        Ok(())
    }
}

pub(crate) fn project_closure(
    root: &OciVerifiedPackage,
    dependencies: &[OciVerifiedPackage],
) -> Result<VerifiedPackageClosure, Error> {
    Ok(VerifiedPackageClosure::new(
        project_package(root)?,
        dependencies
            .iter()
            .map(project_package)
            .collect::<Result<Vec<_>, _>>()?,
    ))
}

pub(crate) fn project_package(
    package: &OciVerifiedPackage,
) -> Result<DomainVerifiedPackage, Error> {
    let kind = DefinitionKind::try_from_product(&package.kind)?;
    let root_path = match kind {
        DefinitionKind::VerificationProcedure => "verification-procedure.yaml",
        DefinitionKind::VerificationActivity => "verification-activity.yaml",
        DefinitionKind::VerificationAction => "verification-action.yaml",
    };
    let semantic_root = package.files.get(root_path).ok_or_else(|| {
        Error::for_system(
            Kind::ProcessingFailure,
            "verified package omits its semantic root",
        )
    })?;
    let semantic_root_projection =
        crate::gateway_driver::controlled_document_projection::project_yaml(semantic_root)
            .map(crate::gateway_driver::controlled_document_projection::from_json)
            .map_err(|_| {
                Error::for_user(
                    Kind::InvalidInput,
                    "verified package semantic root failed controlled projection",
                )
            })?;
    let admitted = nape_domain::service::definition_admission::admit_build_definition(
        &semantic_root_projection,
        &package.identity.package,
    )
    .map_err(|_| {
        Error::for_user(
            Kind::InvalidInput,
            "verified package semantic root failed Product admission",
        )
    })?;
    let lock = PackageLock::new(
        package.lock.profile_version.clone(),
        package.lock.lock_version.clone(),
        PackageReleasePurl::try_new(&package.lock.root)?,
        package
            .lock
            .nodes
            .iter()
            .map(|node| {
                Ok(PackageLockNode {
                    identity: PackageReleaseIdentity::new(
                        PackageReleasePurl::try_new(&node.package)?,
                        ManifestDigest::try_new(&node.manifest_digest)?,
                    ),
                })
            })
            .collect::<Result<Vec<_>, Error>>()?,
        package
            .lock
            .edges
            .iter()
            .map(|edge| {
                Ok(PackageLockEdge {
                    from: PackageReleasePurl::try_new(&edge.from)?,
                    to: PackageReleasePurl::try_new(&edge.to)?,
                    use_selector: edge.use_selector.clone(),
                })
            })
            .collect::<Result<Vec<_>, Error>>()?,
    );
    let file_digests = package
        .files
        .iter()
        .map(|(path, bytes)| {
            (
                path.clone(),
                format!("sha256:{}", hex::encode(Sha256::digest(bytes))),
            )
        })
        .collect::<BTreeMap<_, _>>();
    DomainVerifiedPackage::try_projected_with_lock(
        PackageReleaseIdentity::new(
            PackageReleasePurl::try_new(&package.identity.package)?,
            ManifestDigest::try_new(&package.identity.manifest_digest)?,
        ),
        kind,
        admitted.build.name,
        "application/yaml",
        semantic_root.clone(),
        semantic_root_projection,
        package.files.clone(),
        file_digests,
        Vec::new(),
        lock,
    )
}

pub(crate) fn translate_package_error(
    error: attestify_oci_oss::PackageError,
) -> Result<nape_domain::diagnostic::NapeDiagnostic, Error> {
    nape_domain::diagnostic::NapeDiagnostic::try_new(
        error.diagnostic_code,
        error.reason_template,
        error.phase,
        error.detail,
    )
}

pub(crate) fn translate_oci_error(
    error: attestify_oci_oss::registry::OciError,
) -> Result<nape_domain::diagnostic::NapeDiagnostic, Error> {
    nape_domain::diagnostic::NapeDiagnostic::try_new(
        error.diagnostic_code,
        error.reason_template,
        error.phase,
        error.detail,
    )
}

fn key(package: &str, digest: &str) -> String {
    format!("{package}\n{digest}")
}
