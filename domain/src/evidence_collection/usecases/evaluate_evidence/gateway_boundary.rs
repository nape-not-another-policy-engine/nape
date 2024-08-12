use std::collections::HashMap;
use std::path::{Path};
use nape_kernel::error::{Error, Kind};
use nape_kernel::values::specification::file_path::FilePath;


/// The file path to an evidence file.
///
pub type EvidenceFilePath = FilePath;

/// The file path to a control action test file
/// .
pub type TestFilePath = FilePath;

pub mod request {
    use nape_kernel::values::specification::v1_0_0::assurance_procedure::AssuranceProcedure;
    use super::*;

    /// Contains the mapping of evidence files to the control action test files that each evidence file should be evaluated against.
    #[derive(Clone, Debug, Default)]
    pub struct EvaluationFiles {
        pub file_map: HashMap<EvidenceFilePath, Vec<TestFilePath>>
    }

    impl EvaluationFiles {

        // TODO - Test the EvaluationFiles::from method with the updates to the signature.
        // TODO - look into changing this into a builder, then added a with_canonical_algorithm method to give the option of makeing the file path based upon something like hhome root, then look at adding this same algorithm to the AssurnaceReportBuilder for the evaluate_and_report usecase.
        /// Create a new instance of the [`EvaluationFiles`] struct.
        ///
        /// # Arguments
        ///
        /// * `home_root` - The root path of the home directory where the reports, proceudres, and directoris such as evidence reside.
        /// * `procedure` - The assurance procedure that contains the control actions and evidence files.
        ///
        /// # Returns
        ///
        /// * A new instance of the [`EvaluationFiles`] struct or an [`Error`] if the method fails.
        ///
        pub fn from(home_root: &FilePath, procedure : &AssuranceProcedure) -> Result<Self, Error> {

            let mut evidence_actions_tests = HashMap::new();

            for activity in &procedure.activities.list {
                for action in &activity.actions {
                    let test_file_path = combine_paths(home_root, &action.test)?;
                    let evidence_file_path = combine_paths(home_root, &action.evidence)?;

                    let action_tests = evidence_actions_tests.entry(evidence_file_path)
                        .or_insert_with(Vec::new);

                    if !action_tests.contains(&test_file_path) {
                        action_tests.push(test_file_path);
                    }
                }
            }

            Ok(EvaluationFiles { file_map: evidence_actions_tests })
        }

        // TODO - test the add method
        pub fn add(&self, evidence_file: &EvidenceFilePath, test_file: &TestFilePath) -> Self {
            let mut new_file_map = self.file_map.clone();
            new_file_map.entry(evidence_file.clone())
                .or_insert_with(Vec::new)
                .push(test_file.clone());
            EvaluationFiles { file_map: new_file_map }
        }

        // TODO - find all instances of the .file_mape and replace with list()
        pub fn list(&self) -> &HashMap<EvidenceFilePath, Vec<TestFilePath>> {
            &self.file_map
        }
    }
}

pub mod response {

    use super::*;
    use nape_kernel::values::specification::description::Description;
    use nape_kernel::values::specification::outcome::Outcome;

    /// Contains the results of the evaluation of an evidence file against one or more control actions.
    #[derive(Clone, Debug, Default)]
    pub struct EvaluationResults {
        pub results: HashMap<EvidenceFilePath, HashMap<TestFilePath, TestResult>>
    }

    impl EvaluationResults {

        pub fn add_result(&self, evidence_file: &EvidenceFilePath, test_file: &TestFilePath, test_result: TestResult) -> Self {
            let mut new_results = self.results.clone();
            new_results.entry(evidence_file.clone())
                .or_insert_with(HashMap::new)
                .insert(test_file.clone(), test_result);
            EvaluationResults {  results: new_results }
        }

        pub fn result_for(&self, evidence_file: &EvidenceFilePath, test_file: &TestFilePath ) -> Option<TestResult> {
            self.results
                .get(evidence_file)
                .and_then(|results| results.get(test_file))
                .cloned()
        }

    }

    /// Contains the results of the evaluation of an evidence file against a single control action.
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct TestResult {
        pub outcome: Outcome,
        pub reason: Description,
    }

    impl TestResult {

        pub fn try_from(outcome: &str, reason: &str,) -> Result<Self, Error> {
            let valid_outcome = Outcome::try_from(outcome)?;
            let valid_reason = Description::try_from(reason)?;
            Ok(TestResult {
                outcome: valid_outcome,
                reason: valid_reason
            })
        }

        pub fn reason(&self) -> String {
            self.reason.value.clone()
        }
    }
}

// TODO - TEMP SOLUTION - keep this logic here and reuse it as necessary to create the canonical path a test or evidnce file path untiil a design has been created to handle this logic in a more centralized manner.
pub fn combine_paths(root: &FilePath, action_path: &FilePath) -> Result<FilePath, Error> {

    let combined_path = format!("{}/{}", root.as_str(), action_path.as_str());
    // todo - add checks check root does not end with a "/"
    // todo - add checks check action does not start with a "."
    // todo - add checks check action does start with a /

    if Path::new(&combined_path).is_absolute() || Path::new(&combined_path).is_relative() {
        FilePath::try_from(&combined_path)
    } else {
        Err(Error::for_system(Kind::InvalidInput, format!("The path '{}' is not a valid path.  Please check root path '{}' and relative path '{}' for issues.", combined_path, root.as_str(), action_path.as_str())))
    }

    // TODO - Fix this - It is check if the path exists, and IO don't want to do that.  I just want to conbine the paths.  FIX - Pass the home path instead of  the test or evidence root path this makes it easier
    // let combined_path = root_path.join(action_path.strip_prefix(action_first_component).unwrap());
    // let binding = combined_path.canonicalize()
    //     .map_err(|e| Error::for_system(Kind::ProcessingFailure, format!("Could not create canonical path: {}", e)))?;
    // let canonical_path = binding
    //     .to_str().ok_or_else(|| Error::for_system(Kind::ProcessingFailure, String::from("Could not convert canonical path to string")))?;
    //
    // FilePath::try_from(canonical_path)
}
