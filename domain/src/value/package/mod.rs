//! Exact Attestify package-release identities.

#[cfg(test)]
mod tests;

use kernel_oss::error::{Error, Kind};

use super::bounded_nonempty;
use crate::value::{
    controlled_value::ControlledValue, definition::DefinitionKind, evidence::EvidenceRequirement,
};
use std::collections::BTreeMap;

/// One exact versioned `pkg:attestify` PURL.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PackageReleasePurl(String);

impl PackageReleasePurl {
    /// Creates an exact versioned Attestify package PURL.
    pub fn try_new(value: impl Into<String>) -> Result<Self, Error> {
        let value = bounded_nonempty(value, "package PURL", 2_048)?;
        if !value.starts_with("pkg:attestify/") || !value.contains('@') {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "package PURL must be an exact versioned pkg:attestify identity",
            ));
        }
        Ok(Self(value))
    }

    /// Returns the exact PURL.
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// One exact lowercase SHA-256 manifest digest.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ManifestDigest(String);

impl ManifestDigest {
    /// Creates an exact manifest digest.
    pub fn try_new(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into();
        let valid = value.len() == 71
            && value.starts_with("sha256:")
            && value[7..]
                .chars()
                .all(|character| character.is_ascii_digit() || ('a'..='f').contains(&character));
        if !valid {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "manifest digest must be lowercase sha256:<64 hex>",
            ));
        }
        Ok(Self(value))
    }

    /// Returns the exact digest.
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// The complete exact identity required to acquire a package release.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PackageReleaseIdentity {
    purl: PackageReleasePurl,
    manifest_digest: ManifestDigest,
}

impl PackageReleaseIdentity {
    /// Creates an exact package-release identity.
    pub fn new(purl: PackageReleasePurl, manifest_digest: ManifestDigest) -> Self {
        Self {
            purl,
            manifest_digest,
        }
    }

    /// Returns the exact package PURL.
    pub fn purl(&self) -> &PackageReleasePurl {
        &self.purl
    }

    /// Returns the exact expected manifest digest.
    pub fn manifest_digest(&self) -> &ManifestDigest {
        &self.manifest_digest
    }
}

/// An Application-owned handle to one verified local build result.
pub type LocalPackageHandle = super::external_resource_handle::ExternalResourceHandle;

/// One exact verified package observation returned by an acquisition driver.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedPackage {
    identity: PackageReleaseIdentity,
    kind: DefinitionKind,
    name: String,
    semantic_root_media_type: String,
    semantic_root_bytes: Vec<u8>,
    semantic_root: ControlledValue,
    files: BTreeMap<String, Vec<u8>>,
    file_digests: BTreeMap<String, String>,
    evidence_requirements: Vec<EvidenceRequirement>,
    lock: PackageLock,
}

impl VerifiedPackage {
    /// Creates a transport-neutral verified package observation.
    pub fn try_new(
        identity: PackageReleaseIdentity,
        kind: DefinitionKind,
        name: impl Into<String>,
        semantic_root_media_type: impl Into<String>,
        semantic_root_bytes: Vec<u8>,
        files: BTreeMap<String, Vec<u8>>,
        evidence_requirements: Vec<EvidenceRequirement>,
    ) -> Result<Self, Error> {
        Self::try_with_lock(
            identity,
            kind,
            name,
            semantic_root_media_type,
            semantic_root_bytes,
            files,
            evidence_requirements,
            PackageLock::empty(),
        )
    }

    /// Creates a transport-neutral verified package observation with its
    /// exact, already-verified dependency Lock projection.
    #[allow(clippy::too_many_arguments)]
    pub fn try_with_lock(
        identity: PackageReleaseIdentity,
        kind: DefinitionKind,
        name: impl Into<String>,
        semantic_root_media_type: impl Into<String>,
        semantic_root_bytes: Vec<u8>,
        files: BTreeMap<String, Vec<u8>>,
        evidence_requirements: Vec<EvidenceRequirement>,
        lock: PackageLock,
    ) -> Result<Self, Error> {
        Ok(Self {
            identity,
            kind,
            name: bounded_nonempty(name, "verified package definition name", 128)?,
            semantic_root_media_type: bounded_nonempty(
                semantic_root_media_type,
                "semantic-root media type",
                128,
            )?,
            semantic_root_bytes,
            semantic_root: ControlledValue::Null,
            files,
            file_digests: BTreeMap::new(),
            evidence_requirements,
            lock,
        })
    }

    /// Creates a transport-neutral package with its Application-projected
    /// semantic root and independently observed file digests.
    #[allow(clippy::too_many_arguments)]
    pub fn try_projected_with_lock(
        identity: PackageReleaseIdentity,
        kind: DefinitionKind,
        name: impl Into<String>,
        semantic_root_media_type: impl Into<String>,
        semantic_root_bytes: Vec<u8>,
        semantic_root: ControlledValue,
        files: BTreeMap<String, Vec<u8>>,
        file_digests: BTreeMap<String, String>,
        evidence_requirements: Vec<EvidenceRequirement>,
        lock: PackageLock,
    ) -> Result<Self, Error> {
        if files.keys().ne(file_digests.keys()) {
            return Err(Error::for_system(
                Kind::ProcessingFailure,
                "verified package file bytes and digest inventories differ",
            ));
        }
        Ok(Self {
            identity,
            kind,
            name: bounded_nonempty(name, "verified package definition name", 128)?,
            semantic_root_media_type: bounded_nonempty(
                semantic_root_media_type,
                "semantic-root media type",
                128,
            )?,
            semantic_root_bytes,
            semantic_root,
            files,
            file_digests,
            evidence_requirements,
            lock,
        })
    }

    /// Returns the exact package identity.
    pub fn identity(&self) -> &PackageReleaseIdentity {
        &self.identity
    }

    /// Returns the semantic-root kind.
    pub fn kind(&self) -> DefinitionKind {
        self.kind
    }

    /// Returns the definition-owned name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the controlled semantic-root media type.
    pub fn semantic_root_media_type(&self) -> &str {
        &self.semantic_root_media_type
    }

    /// Returns the exact semantic-root bytes.
    pub fn semantic_root_bytes(&self) -> &[u8] {
        &self.semantic_root_bytes
    }

    /// Returns the transport-neutral controlled semantic-root projection.
    pub fn semantic_root(&self) -> &ControlledValue {
        &self.semantic_root
    }

    /// Returns the exact verified package-owned file inventory.
    pub fn files(&self) -> &BTreeMap<String, Vec<u8>> {
        &self.files
    }

    /// Returns the independently observed digest for one verified file.
    pub fn file_digest(&self, path: &str) -> Option<&str> {
        self.file_digests.get(path).map(String::as_str)
    }

    /// Returns the package-owned evidence requirements.
    pub fn evidence_requirements(&self) -> &[EvidenceRequirement] {
        &self.evidence_requirements
    }

    /// Returns the exact verified package Lock projection.
    pub fn lock(&self) -> &PackageLock {
        &self.lock
    }
}

/// One full verified root-plus-dependency closure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedPackageClosure {
    root: VerifiedPackage,
    dependencies: Vec<VerifiedPackage>,
}

impl VerifiedPackageClosure {
    /// Creates a closure whose dependencies are already canonical ordered.
    pub fn new(root: VerifiedPackage, dependencies: Vec<VerifiedPackage>) -> Self {
        Self { root, dependencies }
    }

    /// Returns the verified root.
    pub fn root(&self) -> &VerifiedPackage {
        &self.root
    }

    /// Returns canonical ordered dependencies.
    pub fn dependencies(&self) -> &[VerifiedPackage] {
        &self.dependencies
    }
}

/// One package Lock aggregate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LockSummary {
    /// Number of dependency nodes.
    pub node_count: u64,
    /// Number of dependency edges.
    pub edge_count: u64,
}

/// One exact transport-neutral package Lock.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageLock {
    profile_version: String,
    lock_version: String,
    root: PackageReleasePurl,
    nodes: Vec<PackageLockNode>,
    edges: Vec<PackageLockEdge>,
}

impl PackageLock {
    /// Creates the placeholder used only by narrow unit fixtures that do not
    /// exercise package graph semantics.
    pub fn empty() -> Self {
        Self {
            profile_version: String::new(),
            lock_version: String::new(),
            root: PackageReleasePurl(String::new()),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Creates an exact verified Lock projection.
    pub fn new(
        profile_version: impl Into<String>,
        lock_version: impl Into<String>,
        root: PackageReleasePurl,
        nodes: Vec<PackageLockNode>,
        edges: Vec<PackageLockEdge>,
    ) -> Self {
        Self {
            profile_version: profile_version.into(),
            lock_version: lock_version.into(),
            root,
            nodes,
            edges,
        }
    }

    /// Returns the profile version.
    pub fn profile_version(&self) -> &str {
        &self.profile_version
    }
    /// Returns the Lock format version.
    pub fn lock_version(&self) -> &str {
        &self.lock_version
    }
    /// Returns the package whose direct dependency graph this Lock owns.
    pub fn root(&self) -> &PackageReleasePurl {
        &self.root
    }
    /// Returns canonical dependency Nodes.
    pub fn nodes(&self) -> &[PackageLockNode] {
        &self.nodes
    }
    /// Returns canonical dependency Edges.
    pub fn edges(&self) -> &[PackageLockEdge] {
        &self.edges
    }
    /// Returns whether this Lock has no dependency graph.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty() && self.edges.is_empty()
    }
}

/// One exact dependency identity in a verified Lock.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageLockNode {
    /// Exact dependency release identity.
    pub identity: PackageReleaseIdentity,
}

/// One exact contextual use Edge in a verified Lock.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageLockEdge {
    /// Exact parent package PURL.
    pub from: PackageReleasePurl,
    /// Exact dependency package PURL.
    pub to: PackageReleasePurl,
    /// Definition-owned or contextual use selector.
    pub use_selector: String,
}
