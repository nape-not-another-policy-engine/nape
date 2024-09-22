// use std::fs;
// use nape_domain::evidence_collection::usecases::collect_evidence::usecase::CollectEvidenceRequest;
// use crate::usecase_configuration::collect_evidence;
//
// // TODO - This is a flakey test because in intrinsically relies on the config file in the file system which may have been created at the time of the test.  FIGURE OUT HOW TO DE-FLAKE THIS, OR delete it since what std_fs is really doing is just returining a confugured function pointer.
// #[test]
// fn collect_evidence_std_fs_success() {
//
//     /*  Arrange */
//
//     // create the evidence file to move
//     let file_data = b"test data";
//     let file_path = "my-tool-report-collect_evidence_std_fs_success.json";
//     fs::write(&file_path, file_data)
//         .expect("Failed to write file data to file 'my-tool-report.json'.");
//
//     let request = CollectEvidenceRequest {
//         action_name: "peer-review",
//         file_path: "my-tool-report-collect_evidence_std_fs_success.json",
//         file_name: Some("peer-review-collect_evidence_std_fs_success.json"),
//     };
//
//     let usecase = collect_evidence::std_fs_factory();
//
//     /*  Act  */
//
//     let result = usecase(&request);
//
//     /* Assert */
//
//     assert!(result.is_ok(), "An error occurred when one was not expected: {:?}", result.err());
//     let copied_file = result.unwrap();
//
//     // read the copied file & assert the data has not been altered
//     let copied_data = fs::read(&copied_file.file_location)
//         .expect("Failed to read the copied file data.");
//     assert_eq!(file_data.to_vec(), copied_data, "Copied file data does not match original data");
//
//     // Clean up test directory and the test file
//     fs::remove_file("my-tool-report-collect_evidence_std_fs_success.json")
//         .expect("Failed to remove the original file 'my-tool-report.json'.");
// }