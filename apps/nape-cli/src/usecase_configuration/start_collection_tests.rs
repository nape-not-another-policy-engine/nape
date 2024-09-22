use std::fs;
use std::path::Path;
use nape_domain::evidence_collection::usecases::start_collection::usecase_boundary::request::{StartProcedureBuilder};
use crate::usecase_configuration::start_collection::factory_std_fs_git2;

/// Large Test
#[test]
fn factory_std_fs_git2_success() {
    let request = StartProcedureBuilder::default()
        .start_at(1714646108364)
        .api_version("1.0.0")
        .subject_nrn("nrn:sourcecode:example")
        .subject_id("123456789")
        .procedure_repository("https://github.com/nape-dev/catalog.git")
        .procedure_directory("rust_ci/sourcecode_integration")
        .add_metadata("key-1", "value-1")
        .try_build().unwrap();

    let usecase = factory_std_fs_git2();
    let result = usecase(request);

    assert!(result.is_ok(), "Expected Ok, but got: {:?}", result);
    assert!(Path::new("nrn_sourcecode_example/1714646108364").exists(), "The subject/subject_id directory does not exist.");
    assert!(Path::new("nrn_sourcecode_example/1714646108364/assurance_procedure.yaml").exists(), "The assurance_procedure.yaml does not exist.");
    assert!(Path::new("nrn_sourcecode_example/1714646108364/evidence").exists(), "The evidence directory does not exist.");
    assert!(Path::new("nrn_sourcecode_example/1714646108364/activity").exists(), "The activity test directory does not exist.");
    assert!(Path::new("nrn_sourcecode_example/1714646108364/activity/peer_review").exists(), "The activity/peer_review directory does not exist.");
    assert!(Path::new("nrn_sourcecode_example/1714646108364/activity/peer_review/at_least_two_reviewers.py").exists(), "The peer_review/at_least_two_reviewers.py does not exist.");
    assert!(Path::new("nrn_sourcecode_example/1714646108364/activity/peer_review/requester_not_a_reviewer.py").exists(), "The peer_review/requester_not_a_reviewer.py does not exist.");

    let directory_list = result.unwrap().directory_list;
    assert_eq!(directory_list.paths.len(), 5, "Expected 4 directories, but got: {}", directory_list.paths.len());
    assert!(directory_list.try_get("home").unwrap().ends_with("nrn_sourcecode_example/1714646108364"), "The 'home' directory entry isn't what's expected.");
    assert!(directory_list.try_get("evidence").unwrap().ends_with("nrn_sourcecode_example/1714646108364/evidence"), "The 'evidence' directory entry isn't what's expected.");
    assert!(directory_list.try_get("activity-test").unwrap().ends_with("nrn_sourcecode_example/1714646108364/activity"), "The 'activity-test' directory entry isn't what's expected.");
    assert!(directory_list.try_get("temp").unwrap().ends_with("nrn_sourcecode_example/1714646108364/temp"), "The 'temp' directory entry isn't what's expected.");
    assert!(directory_list.try_get("assurance-procedure-file").unwrap().ends_with("nrn_sourcecode_example/1714646108364/assurance_procedure.yaml"), "The 'assurance-procedure-file' directory entry isn't what's expected.");

    // Clean up the activity test directory after the test is run
    fs::remove_dir_all("nrn_sourcecode_example")
        .expect("Failed to remove 'nrn_sourcecode_example' directory");
}