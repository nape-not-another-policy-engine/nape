pub mod assurance_report;
pub mod assurance_procedure;
pub mod traits;
pub mod v1_0_0;

pub mod api_version;
pub mod description;
pub mod file_path;
pub mod subject;
pub mod subject_id;
pub mod metadata;
pub mod name;
pub mod outcome;
pub mod procedure;
pub mod repository_link;
pub mod short_description;
pub mod kind;


#[cfg(test)] mod api_version_tests;
#[cfg(test)] mod description_tests;
#[cfg(test)] mod file_path_tests;
#[cfg(test)]  mod metadata_tests;
#[cfg(test)] mod name_tests;
#[cfg(test)] mod subject_tests;
#[cfg(test)] mod subject_id_tests;
#[cfg(test)] mod outcome_tests;
#[cfg(test)] mod procedure_tests;
#[cfg(test)] mod repository_link_tests;
#[cfg(test)] mod short_description_tests;
#[cfg(test)] mod kind_tests;

