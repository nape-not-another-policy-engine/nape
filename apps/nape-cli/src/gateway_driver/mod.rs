//! Application-owned drivers for NAPE Domain gateway seams.

pub mod action_evaluation_nape_evaluator;
pub mod application_identity_local;
pub mod authored_definition_source_local;
pub mod controlled_document_projection;
pub(crate) mod current_run_state;
pub mod current_utc_timestamp_system;
pub mod current_verification_acquisition_local;
pub mod definition_package_build_attestify_oci;
pub(crate) mod definition_package_profile_attestify_oci;
pub mod definition_package_publication_attestify_oci;
pub mod evidence_acquisition_local;
pub mod local_definition_package_acquisition_attestify_oci;
mod local_file;
pub mod new_identity_ulid;
pub mod oci_definition_package_resolution_attestify_oci;
pub(crate) mod package_support;
pub mod registry_configuration_attestify_oci;
pub mod restricted_schema_validation;
pub mod verification_evidence_commit_local;
pub mod verification_outcome_commit_local;
pub mod verification_start_commit_local;
pub mod verification_subject_acquisition_local;
