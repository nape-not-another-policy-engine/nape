use crate::evidence_collection::usecases::evaluate_evidence::gateway_boundary::{EvidenceFilePath, TestFilePath};
use crate::evidence_collection::usecases::evaluate_evidence::gateway_boundary::response::EvaluationResults;
use crate::evidence_collection::usecases::evaluate_evidence::gateway_boundary::response::TestResult;



mod test_result_tests {
    use super::*;
    use nape_kernel::values::specification::outcome::Outcome;
    /* Happy Path */


    #[test]
    fn try_from_pass_success() {
        let test_result = TestResult::try_from("pass", "The pass reason.").unwrap();
        assert_eq!(test_result.outcome, Outcome::PASS);
        assert_eq!(test_result.reason(), "The pass reason.");
    }

    #[test]
    fn try_from_fail_success() {
        let test_result = TestResult::try_from("fail", "The fail reason.").unwrap();
        assert_eq!(test_result.outcome, Outcome::FAIL);
        assert_eq!(test_result.reason(), "The fail reason.");
    }

    #[test]
    fn try_from_inconclusive_success() {
        let test_result = TestResult::try_from("inconclusive", "The inconclusive reason.").unwrap();
        assert_eq!(test_result.outcome, Outcome::INCONCLUSIVE);
        assert_eq!(test_result.reason(), "The inconclusive reason.");
    }

    #[test]
    fn try_from_error_success() {
        let test_result = TestResult::try_from("error", "The error reason.").unwrap();
        assert_eq!(test_result.outcome, Outcome::ERROR);
        assert_eq!(test_result.reason(), "The error reason.");
    }

    /* Error Path */

    #[test]
    fn try_from_empty_outcome_error() {
        let test_result = TestResult::try_from("", "The reason.");
        assert!(test_result.is_err());
    }

    #[test]
    fn try_from_invalid_outcome_error() {
        let test_result = TestResult::try_from("invalid", "The reason.");
        assert!(test_result.is_err());
    }

    #[test]
    fn try_from_empty_reason_error() {
        let test_result = TestResult::try_from("pass", "");
        assert!(test_result.is_err());
    }

}

mod request  {
    use super::*;
    use std::collections::HashMap;
    use nape_kernel::values::specification::assurance_procedure::action::Action;
    use nape_kernel::values::specification::assurance_procedure::activity::Activity;
    use nape_kernel::values::specification::file_path::FilePath;
    use nape_kernel::values::specification::v1_0_0::assurance_procedure::AssuranceProcedure;
    use crate::evidence_collection::usecases::evaluate_evidence::gateway_boundary::request::EvaluationFiles;

    /// Testing where there are no repeating evidence files, or test files, and each evidence file has one test file associated with it.
    ///
    #[test]
    fn from_procedure_many_activity_many_tests_success() {

        let  activity1 = Activity::new("activity-1", "short description", "long description", ).unwrap();
        let  activity2 = Activity::new("activity-2", "short description", "long description", ).unwrap();
        let  activity3 = Activity::new("activity-3", "short description", "long description", ).unwrap();

        let a1_action1 = Action::builder().name("action-1-1").short_description("short description").long_description("long description").test_file_path("activity/test_dir/test_file_1_1").evidence_file_path("evidence/test_dir/evidence_file_1_1").try_build().unwrap();
        let a1_action2 = Action::builder().name("action-1-2").short_description("short description").long_description("long description").test_file_path("activity/test_dir/test_file_1_2").evidence_file_path("evidence/test_dir/evidence_file_1_2").try_build().unwrap();
        let a1_action3 = Action::builder().name("action-1-3").short_description("short description").long_description("long description").test_file_path("activity/test_dir/test_file_1_3").evidence_file_path("evidence/test_dir/evidence_file_1_3").try_build().unwrap();
        let activity1 = activity1.add(a1_action1).add(a1_action2).add(a1_action3);

        let a2_action1 = Action::builder().name("action-2-1").short_description("short description").long_description("long description").test_file_path("activity/test_dir/test_file_2_1").evidence_file_path("evidence/test_dir/evidence_file_2_1").try_build().unwrap();
        let a2_action2 = Action::builder().name("action-2-2").short_description("short description").long_description("long description").test_file_path("activity/test_dir/test_file_2_2").evidence_file_path("evidence/test_dir/evidence_file_2_2").try_build().unwrap();
        let a2_action3 = Action::builder().name("action-2-3").short_description("short description").long_description("long description").test_file_path("activity/test_dir/test_file_2_3").evidence_file_path("evidence/test_dir/evidence_file_2_3").try_build().unwrap();
        let activity2 = activity2.add(a2_action1).add(a2_action2).add(a2_action3);

        let a3_action1 = Action::builder().name("action-3-1").short_description("short description").long_description("long description").test_file_path("activity/test_dir/test_file_3_1").evidence_file_path("evidence/test_dir/evidence_file_3_1").try_build().unwrap();
        let a3_action2 = Action::builder().name("action-3-2").short_description("short description").long_description("long description").test_file_path("activity/test_dir/test_file_3_2").evidence_file_path("evidence/test_dir/evidence_file_3_2").try_build().unwrap();
        let a3_action3 = Action::builder().name("action-3-3").short_description("short description").long_description("long description").test_file_path("activity/test_dir/test_file_3_3").evidence_file_path("evidence/test_dir/evidence_file_3_3").try_build().unwrap();
        let activity3 = activity3.add(a3_action1).add(a3_action2).add(a3_action3);

        let procedure = AssuranceProcedure::builder()
            .api_version("1.0.0")
            .procedure_info("nrn:sourcecode::example", "A Short Desc.", "This is an example procedure")
            .add_activity(&activity1)
            .add_activity(&activity2)
            .add_activity(&activity3)
            .try_build().unwrap();

        let home_root = FilePath::from("/Users/someone/procedure/home");
        let evidence_action_files = EvaluationFiles::from(&home_root,  &procedure).unwrap();

        // Expected result
        let mut expected_tests = HashMap::new();
        expected_tests.insert(
            EvidenceFilePath::from("/Users/someone/procedure/home/evidence/test_dir/evidence_file_1_1"),
            vec![TestFilePath::from("/Users/someone/procedure/home/activity/test_dir/test_file_1_1")],
        );
        expected_tests.insert(
            EvidenceFilePath::from("/Users/someone/procedure/home/evidence/test_dir/evidence_file_1_2"),
            vec![TestFilePath::from("/Users/someone/procedure/home/activity/test_dir/test_file_1_2")],
        );
        expected_tests.insert(
            EvidenceFilePath::from("/Users/someone/procedure/home/evidence/test_dir/evidence_file_1_3"),
            vec![TestFilePath::from("/Users/someone/procedure/home/activity/test_dir/test_file_1_3")],
        );
        expected_tests.insert(
            EvidenceFilePath::from("/Users/someone/procedure/home/evidence/test_dir/evidence_file_2_1"),
            vec![TestFilePath::from("/Users/someone/procedure/home/activity/test_dir/test_file_2_1")],
        );
        expected_tests.insert(
            EvidenceFilePath::from("/Users/someone/procedure/home/evidence/test_dir/evidence_file_2_2"),
            vec![TestFilePath::from("/Users/someone/procedure/home/activity/test_dir/test_file_2_2")],
        );
        expected_tests.insert(
            EvidenceFilePath::from("/Users/someone/procedure/home/evidence/test_dir/evidence_file_2_3"),
            vec![TestFilePath::from("/Users/someone/procedure/home/activity/test_dir/test_file_2_3")],
        );
        expected_tests.insert(
            EvidenceFilePath::from("/Users/someone/procedure/home/evidence/test_dir/evidence_file_3_1"),
            vec![TestFilePath::from("/Users/someone/procedure/home/activity/test_dir/test_file_3_1")],
        );
        expected_tests.insert(
            EvidenceFilePath::from("/Users/someone/procedure/home/evidence/test_dir/evidence_file_3_2"),
            vec![TestFilePath::from("/Users/someone/procedure/home/activity/test_dir/test_file_3_2")],
        );
        expected_tests.insert(
            EvidenceFilePath::from("/Users/someone/procedure/home/evidence/test_dir/evidence_file_3_3"),
            vec![TestFilePath::from("/Users/someone/procedure/home/activity/test_dir/test_file_3_3")],
        );

        assert_eq!(expected_tests, evidence_action_files.file_map)
    }

    /// # Test Description
    /// Testing when there are repeating evidence files within an activity, although each action has its own unique test file across all actions for every activity within the procedure. No evidence file is repeated across, actions, onluy within the action.
    ///
    #[test]
    fn from_procedure_action_evidence_duplicates_action_test_unique_success() {

        let  activity1 = Activity::new("activity-1", "short description", "long description", ).unwrap();
        let  activity2 = Activity::new("activity-2", "short description", "long description", ).unwrap();
        let  activity3 = Activity::new("activity-3", "short description", "long description", ).unwrap();

        let a1_action1 = Action::builder().name("action-1-1").short_description("short description").long_description("long description").test_file_path("test_file_1_1").evidence_file_path("evidence_file_1_1").try_build().unwrap();
        let a1_action2 = Action::builder().name("action-1-2").short_description("short description").long_description("long description").test_file_path("test_file_1_2").evidence_file_path("evidence_file_1_1").try_build().unwrap();
        let a1_action3 = Action::builder().name("action-1-3").short_description("short description").long_description("long description").test_file_path("test_file_1_3").evidence_file_path("evidence_file_1_2").try_build().unwrap();
        let activity1 = activity1.add(a1_action1).add(a1_action2).add(a1_action3);

        let a2_action1 = Action::builder().name("action-2-1").short_description("short description").long_description("long description").test_file_path("test_file_2_1").evidence_file_path("evidence_file_2_1").try_build().unwrap();
        let a2_action2 = Action::builder().name("action-2-2").short_description("short description").long_description("long description").test_file_path("test_file_2_2").evidence_file_path("evidence_file_2_2").try_build().unwrap();
        let a2_action3 = Action::builder().name("action-2-3").short_description("short description").long_description("long description").test_file_path("test_file_2_3").evidence_file_path("evidence_file_2_2").try_build().unwrap();
        let activity2 = activity2.add(a2_action1).add(a2_action2).add(a2_action3);

        let a3_action1 = Action::builder().name("action-3-1").short_description("short description").long_description("long description").test_file_path("test_file_3_1").evidence_file_path("evidence_file_3_1").try_build().unwrap();
        let a3_action2 = Action::builder().name("action-3-2").short_description("short description").long_description("long description").test_file_path("test_file_3_2").evidence_file_path("evidence_file_3_1").try_build().unwrap();
        let a3_action3 = Action::builder().name("action-3-3").short_description("short description").long_description("long description").test_file_path("test_file_3_3").evidence_file_path("evidence_file_3_3").try_build().unwrap();
        let activity3 = activity3.add(a3_action1).add(a3_action2).add(a3_action3);

        let procedure = AssuranceProcedure::builder()
            .api_version("1.0.0")
            .procedure_info("nrn:sourcecode::example", "A Short Desc.", "This is an example procedure")
            .add_activity(&activity1)
            .add_activity(&activity2)
            .add_activity(&activity3)
            .try_build().unwrap();

        let home_root = FilePath::from("/Users/someone/procedure/home");
        let evidence_action_files = EvaluationFiles::from(&home_root,  &procedure).unwrap();

        // Expected result
        let mut expected_tests = HashMap::new();

        expected_tests.insert(
            EvidenceFilePath::from("/Users/someone/procedure/home/evidence_file_1_1"),
            vec![
                TestFilePath::from("/Users/someone/procedure/home/test_file_1_1"),
                TestFilePath::from("/Users/someone/procedure/home/test_file_1_2") ],
        );
        expected_tests.insert(
            EvidenceFilePath::from("/Users/someone/procedure/home/evidence_file_1_2"),
            vec![TestFilePath::from("/Users/someone/procedure/home/test_file_1_3")],
        );

        expected_tests.insert(
            EvidenceFilePath::from("/Users/someone/procedure/home/evidence_file_2_1"),
            vec![TestFilePath::from("/Users/someone/procedure/home/test_file_2_1") ],
        );
        expected_tests.insert(
            EvidenceFilePath::from("/Users/someone/procedure/home/evidence_file_2_2"),
            vec![
                TestFilePath::from("/Users/someone/procedure/home/test_file_2_2"),
                TestFilePath::from("/Users/someone/procedure/home/test_file_2_3") ],
        );

        expected_tests.insert(
            EvidenceFilePath::from("/Users/someone/procedure/home/evidence_file_3_1"),
            vec![
                TestFilePath::from("/Users/someone/procedure/home/test_file_3_1"),
                TestFilePath::from("/Users/someone/procedure/home/test_file_3_2") ],
        );
        expected_tests.insert(
            EvidenceFilePath::from("/Users/someone/procedure/home/evidence_file_3_3"),
            vec![TestFilePath::from("/Users/someone/procedure/home/test_file_3_3")],
        );

        assert_eq!(expected_tests, evidence_action_files.file_map)
    }

    /// # Test Description
    /// Testing when there are repeating evidence files across all activities within the procedure, although each action has its own unique test file.
    ///
    #[test]
    fn from_procedure_many_activity_duplicate_evidence_many_tests_success() {

        /* Assemble */

        let  activity1 = Activity::new("activity-1", "short description", "long description", ).unwrap();
        let  activity2 = Activity::new("activity-2", "short description", "long description", ).unwrap();
        let  activity3 = Activity::new("activity-3", "short description", "long description", ).unwrap();

        let a1_action1 = Action::builder().name("action-1-1").short_description("short description").long_description("long description").test_file_path("test_file_1_1").evidence_file_path("evidence_file_1_1").try_build().unwrap();
        let a1_action2 = Action::builder().name("action-1-2").short_description("short description").long_description("long description").test_file_path("test_file_1_2").evidence_file_path("evidence_file_2_1").try_build().unwrap();
        let a1_action3 = Action::builder().name("action-1-3").short_description("short description").long_description("long description").test_file_path("test_file_1_3").evidence_file_path("evidence_file_3_1").try_build().unwrap();
        let activity1 = activity1.add(a1_action1).add(a1_action2).add(a1_action3);

        let a2_action1 = Action::builder().name("action-2-1").short_description("short description").long_description("long description").test_file_path("test_file_2_1").evidence_file_path("evidence_file_3_1").try_build().unwrap();
        let a2_action2 = Action::builder().name("action-2-2").short_description("short description").long_description("long description").test_file_path("test_file_2_2").evidence_file_path("evidence_file_1_1").try_build().unwrap();
        let a2_action3 = Action::builder().name("action-2-3").short_description("short description").long_description("long description").test_file_path("test_file_2_3").evidence_file_path("evidence_file_2_1").try_build().unwrap();
        let activity2 = activity2.add(a2_action1).add(a2_action2).add(a2_action3);

        let a3_action1 = Action::builder().name("action-3-1").short_description("short description").long_description("long description").test_file_path("test_file_3_1").evidence_file_path("evidence_file_2_1").try_build().unwrap();
        let a3_action2 = Action::builder().name("action-3-2").short_description("short description").long_description("long description").test_file_path("test_file_3_2").evidence_file_path("evidence_file_3_1").try_build().unwrap();
        let a3_action3 = Action::builder().name("action-3-3").short_description("short description").long_description("long description").test_file_path("test_file_3_3").evidence_file_path("evidence_file_1_1").try_build().unwrap();
        let activity3 = activity3.add(a3_action1).add(a3_action2).add(a3_action3);

        let procedure = AssuranceProcedure::builder()
            .api_version("1.0.0")
            .procedure_info("nrn:sourcecode::example", "A Short Desc.", "This is an example procedure")
            .add_activity(&activity1)
            .add_activity(&activity2)
            .add_activity(&activity3)
            .try_build().unwrap();

        /* Act */
        let home_root = FilePath::from("/Users/someone/procedure/home");
        let evidence_action_files = EvaluationFiles::from(&home_root,  &procedure).unwrap();

        /* Assert */
        let expect_file_1_1 =  vec![
            TestFilePath::from("/Users/someone/procedure/home/test_file_1_1"),
            TestFilePath::from("/Users/someone/procedure/home/test_file_2_2") ,
            TestFilePath::from("/Users/someone/procedure/home/test_file_3_3") ];

        let expect_file_2_1 = vec![
            TestFilePath::from("/Users/someone/procedure/home/test_file_1_2"),
            TestFilePath::from("/Users/someone/procedure/home/test_file_2_3"),
            TestFilePath::from("/Users/someone/procedure/home/test_file_3_1") ];

        let expect_file_3_1 = vec![
            TestFilePath::from("/Users/someone/procedure/home/test_file_1_3"),
            TestFilePath::from("/Users/someone/procedure/home/test_file_2_1"),
            TestFilePath::from("/Users/someone/procedure/home/test_file_3_2") ];

        let evidence_file_1_1 = evidence_action_files.file_map.get(&EvidenceFilePath::from("/Users/someone/procedure/home/evidence_file_1_1")).unwrap();
        assert_eq!(*evidence_file_1_1, expect_file_1_1);

        let evidence_file_2_1 = evidence_action_files.file_map.get(&EvidenceFilePath::from("/Users/someone/procedure/home/evidence_file_2_1")).unwrap();
        assert_eq!(*evidence_file_2_1, expect_file_2_1);

        let evidence_file_3_1 = evidence_action_files.file_map.get(&EvidenceFilePath::from("/Users/someone/procedure/home/evidence_file_3_1")).unwrap();
        assert_eq!(*evidence_file_3_1, expect_file_3_1);
    }

    /// # Test Description
    /// Testing when there are repeating evidence files across all activities, and there are actions that share the same evidence file and test file as other actions.
    ///
    #[test]
    fn from_procedure_many_activity_duplicate_evidence_many_test_duplicate_tests_success() {

        /* Assemble */

        let  activity1 = Activity::new("activity-1", "short description", "long description", ).unwrap();
        let  activity2 = Activity::new("activity-2", "short description", "long description", ).unwrap();
        let  activity3 = Activity::new("activity-3", "short description", "long description", ).unwrap();

        let a1_action1 = Action::builder().name("action-1-1").short_description("short description").long_description("long description").test_file_path("test_file_1_1").evidence_file_path("evidence_file_1_1").try_build().unwrap();
        let a1_action2 = Action::builder().name("action-1-2").short_description("short description").long_description("long description").test_file_path("test_file_1_2").evidence_file_path("evidence_file_1_1").try_build().unwrap();
        let a1_action3 = Action::builder().name("action-1-3").short_description("short description").long_description("long description").test_file_path("test_file_1_3").evidence_file_path("evidence_file_1_2").try_build().unwrap();
        let activity1 = activity1.add(a1_action1).add(a1_action2).add(a1_action3);

        let a2_action1 = Action::builder().name("action-2-1").short_description("short description").long_description("long description").test_file_path("test_file_1_1").evidence_file_path("evidence_file_1_1").try_build().unwrap();
        let a2_action2 = Action::builder().name("action-2-2").short_description("short description").long_description("long description").test_file_path("test_file_2_1").evidence_file_path("evidence_file_1_1").try_build().unwrap();
        let a2_action3 = Action::builder().name("action-2-3").short_description("short description").long_description("long description").test_file_path("test_file_2_3").evidence_file_path("evidence_file_2_3").try_build().unwrap();
        let activity2 = activity2.add(a2_action1).add(a2_action2).add(a2_action3);


        let procedure = AssuranceProcedure::builder()
            .api_version("1.0.0")
            .procedure_info("nrn:sourcecode::example", "A Short Desc.", "This is an example procedure")
            .add_activity(&activity1)
            .add_activity(&activity2)
            .add_activity(&activity3)
            .try_build().unwrap();

        /* Act */
        let home_root = FilePath::from("/Users/someone/procedure/home");
        let evidence_action_files = EvaluationFiles::from(&home_root,  &procedure).unwrap();

        /* Assert */
        let expect_file_1_1 =  vec![
            TestFilePath::from("/Users/someone/procedure/home/test_file_1_1"),
            TestFilePath::from("/Users/someone/procedure/home/test_file_1_2"),
            TestFilePath::from("/Users/someone/procedure/home/test_file_2_1") ];

        let expect_file_1_2 = vec![
            TestFilePath::from("/Users/someone/procedure/home/test_file_1_3")  ];

        let expect_file_2_3 = vec![
            TestFilePath::from("/Users/someone/procedure/home/test_file_2_3") ];

        let evidence_file_1_1 = evidence_action_files.file_map.get(&EvidenceFilePath::from("/Users/someone/procedure/home/evidence_file_1_1")).unwrap();
        assert_eq!(*evidence_file_1_1, expect_file_1_1);

        let evidence_file_1_2 = evidence_action_files.file_map.get(&EvidenceFilePath::from("/Users/someone/procedure/home/evidence_file_1_2")).unwrap();
        assert_eq!(*evidence_file_1_2, expect_file_1_2);

        let evidence_file_2_3 = evidence_action_files.file_map.get(&EvidenceFilePath::from("/Users/someone/procedure/home/evidence_file_2_3")).unwrap();
        assert_eq!(*evidence_file_2_3, expect_file_2_3);
    }

    #[test]
    fn add_success() {

        let test_file1 = TestFilePath::from("new_test_file.yaml");
        let test_file2 = TestFilePath::from("test_file2.yaml");
        let test_file3 = TestFilePath::from("test_file3.yaml");
        let evidence_file1 = EvidenceFilePath::from("new_evidence_file.yaml");
        let evidence_file2 = EvidenceFilePath::from("evidence_file2.yaml");

        let evaluation_files = EvaluationFiles::default();
        let evaluation_files = evaluation_files.add(&evidence_file1, &test_file1);
        let evaluation_files = evaluation_files.add(&evidence_file1, &test_file2);
        let evaluation_files = evaluation_files.add(&evidence_file2, &test_file3);


        let actual_test_files1 = evaluation_files.file_map.get(&EvidenceFilePath::from("new_evidence_file.yaml")).unwrap();
        let expected_test_files1 = vec![TestFilePath::from("new_test_file.yaml"), TestFilePath::from("test_file2.yaml")];

        let actual_test_files2 = evaluation_files.file_map.get(&EvidenceFilePath::from("evidence_file2.yaml")).unwrap();
        let expected_test_files2 = vec![TestFilePath::from("test_file3.yaml")];

        assert_eq!(&expected_test_files1, actual_test_files1);
        assert_eq!(&expected_test_files2, actual_test_files2);

    }

}

mod response {

    use super::*;
    use std::collections::HashMap;
    use nape_kernel::values::specification::file_path::FilePath;
    use nape_kernel::values::specification::outcome::Outcome;

    /* Happy Path */

    #[test]
    fn default_success() {
        let evaluation_results = EvaluationResults::default();
        assert_eq!(evaluation_results.results, HashMap::new());
    }

    #[test]
    fn add_result_success()  {

        /* Assemble */
        let evidence_file = FilePath::from("evidence.txt");
        let evaluation_results = EvaluationResults::default();

        let test_file1 = FilePath::from("test1.txt");
        let test_file2 = FilePath::from("test2.txt");
        let test_file3 = FilePath::from("test3.txt");
        let test_file4 = FilePath::from("test4.txt");

        let result1 = TestResult::try_from("pass", "the pass reason").unwrap();
        let result2 = TestResult::try_from("fail", "the fail reason").unwrap();
        let result3 = TestResult::try_from("inconclusive", "the inconclusive reason").unwrap();
        let result4 = TestResult::try_from("error", "the error reason").unwrap();

        /* Act */
        let evaluation_results = evaluation_results.add_result(&evidence_file, &test_file1, result1);
        let evaluation_results = evaluation_results.add_result(&evidence_file, &test_file2, result2);
        let evaluation_results = evaluation_results.add_result(&evidence_file, &test_file3, result3);
        let evaluation_results = evaluation_results.add_result(&evidence_file, &test_file4, result4);

        /* Assert */
        let retrieve_result1 = evaluation_results.result_for(&evidence_file, &test_file1).unwrap();
        let retrieve_result2 = evaluation_results.result_for(&evidence_file, &test_file2).unwrap();
        let retrieve_result3 = evaluation_results.result_for(&evidence_file, &test_file3).unwrap();
        let retrieve_result4 = evaluation_results.result_for(&evidence_file, &test_file4).unwrap();

        assert_eq!(retrieve_result1.outcome, Outcome::PASS);
        assert_eq!(retrieve_result1.reason(), "the pass reason");

        assert_eq!(retrieve_result2.outcome, Outcome::FAIL);
        assert_eq!(retrieve_result2.reason(), "the fail reason");

        assert_eq!(retrieve_result3.outcome, Outcome::INCONCLUSIVE);
        assert_eq!(retrieve_result3.reason(), "the inconclusive reason");

        assert_eq!(retrieve_result4.outcome, Outcome::ERROR);
        assert_eq!(retrieve_result4.reason(), "the error reason");
    }

    /* Error Path */

    #[test]
    fn result_for_not_found_error() {
        let evidence_file = FilePath::from("evidence.txt");
        let evaluation_results = EvaluationResults::default();
        let test_file = FilePath::from("test.txt");
        let result = evaluation_results.result_for(&evidence_file, &test_file);
        assert!(result.is_none());
    }

}