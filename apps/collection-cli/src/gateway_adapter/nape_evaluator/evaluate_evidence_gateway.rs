use std::process::Command;
use serde::Deserialize;
use nape_domain::evidence_collection::usecases::evaluate_evidence::gateway_boundary::request::EvaluationFiles;
use nape_domain::evidence_collection::usecases::evaluate_evidence::gateway_boundary::response::{EvaluationResults, TestResult};
use nape_kernel::error::{Error, Kind};
use nape_kernel::values::specification::file_path::FilePath;

#[derive(Deserialize)]
struct NapeEvalOutput {
    outcome: String,
    reason: String,
}

impl NapeEvalOutput {
    fn try_to_test_result(&self) -> Result<TestResult, Error> {
        TestResult::try_from(&self.outcome, &self.reason)
            .map_err(|e| Error::for_system(Kind::ProcessingFailure,
                                           format!("Failed to convert nape-eval output to a TestResult. {}", e)))
    }
}

pub fn nape_evidence_evaluator(files: &EvaluationFiles) -> Result<EvaluationResults, Error> {

    verify_cli_install()?;

    let mut results = EvaluationResults::default();

    for (evidence_file, test_files) in files.list() {
        for test_file in test_files {
            let cli_result = invoke_cli(evidence_file, test_file)?;
            let test_result = deserialize_cli_result_into_test_result(&cli_result)?;
            results = results.add_result(&evidence_file, &test_file, test_result);
        }
    }

    Ok(results)
}

fn verify_cli_install() -> Result<(), Error> {

    let check_install = Command::new("nape-eval")
        .arg("--check-install")
        .output()
        .map_err(|e| Error::for_system(Kind::NotFound, format!("Failed to execute the install verification command 'nape-eval --check-install'. {}", e)))?;

    if !check_install.status.success() {
        return Err(Error::for_system(Kind::NotFound, String::from("The 'nape-eval' CLI is not installed. Please verify you have installed the 'nape-eval' CLI and the CLI is reachable by this user.")));
    }

    Ok(())

}

fn invoke_cli(evidence_file: &FilePath, test_file: &FilePath) -> Result<String, Error> {

    let output = Command::new("nape-eval")
        .arg("--evidence").arg(evidence_file.as_str())
        .arg("--test").arg(test_file.as_str())
        .output().map_err(|e| Error::for_system(Kind::ProcessingFailure,
                                                format!("Failed to execute 'nape-eval' cli command. {}", e)))?;


    if !output.status.success() {
        return Err(Error::for_system(Kind::ProcessingFailure,
                                   format!("The 'nape-eval' CLI execution was not a success.  The CLI returned a non-zero exit code.  The CLI output was: {:?}", output.stderr)));
    }

    let output_str =    String::from_utf8(output.stdout)
        .map_err(|e| Error::for_system(Kind::ProcessingFailure,
                                       format!("Failed to parse nape-eval output. {}", e)))?;

    Ok(output_str)
}

fn deserialize_cli_result_into_test_result(cli_result: &str) -> Result<TestResult, Error> {
    let eval_output: NapeEvalOutput = serde_json::from_str(&cli_result)
        .map_err(|e| Error::for_system(Kind::ProcessingFailure,
                                       format!("Failed to deserialize nape-eval output: {}", e)))?;
    let test_result = eval_output.try_to_test_result()?;
    Ok(test_result)
}