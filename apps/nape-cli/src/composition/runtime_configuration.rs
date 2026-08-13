//! Bounded process configuration for exact NAPE Evaluator executables.

use std::path::PathBuf;

use kernel_oss::error::{Error, Kind};

use crate::gateway_driver::action_evaluation_nape_evaluator::{
    contract_v2::{ConsumerFailure, EvaluatorExecutable},
    contract_v3, SelectedEvaluatorExecutable,
};

pub(super) fn select_evaluators() -> Result<SelectedEvaluatorExecutable, Error> {
    let v2 = evaluator_environment("V2")?;
    let v3 = evaluator_environment("V3")?;
    match (v2, v3) {
        (Some((path, record, digest)), Some((path3, record3, digest3))) => {
            let v2 = EvaluatorExecutable::select(path, record, &digest)
                .map_err(evaluator_configuration_error)?;
            let v3 = contract_v3::select_evaluator(path3, record3, &digest3)
                .map_err(evaluator_configuration_error)?;
            Ok(SelectedEvaluatorExecutable::V2AndV3 { v2, v3 })
        }
        (Some((path, record, digest)), None) => Ok(SelectedEvaluatorExecutable::V2(
            EvaluatorExecutable::select(path, record, &digest)
                .map_err(evaluator_configuration_error)?,
        )),
        (None, Some((path, record, digest))) => Ok(SelectedEvaluatorExecutable::V3(
            contract_v3::select_evaluator(path, record, &digest)
                .map_err(evaluator_configuration_error)?,
        )),
        (None, None) => Err(Error::for_user(
            Kind::InvalidInput,
            "no NAPE Evaluator V2 or V3 executable configuration is available",
        )),
    }
}

fn evaluator_environment(version: &str) -> Result<Option<(PathBuf, PathBuf, String)>, Error> {
    let path = std::env::var_os(format!("NAPE_EVALUATOR_{version}_EXECUTABLE"));
    let record = std::env::var_os(format!("NAPE_EVALUATOR_{version}_BUILD_RECORD"));
    let digest = std::env::var(format!("NAPE_EVALUATOR_{version}_BUILD_RECORD_SHA256")).ok();
    match (path, record, digest) {
        (None, None, None) => Ok(None),
        (Some(path), Some(record), Some(digest)) => {
            Ok(Some((PathBuf::from(path), PathBuf::from(record), digest)))
        }
        _ => Err(Error::for_user(
            Kind::InvalidInput,
            format!("NAPE Evaluator {version} configuration is incomplete"),
        )),
    }
}

fn evaluator_configuration_error(error: ConsumerFailure) -> Error {
    Error::for_user(
        Kind::InvalidInput,
        format!("Evaluator configuration failed: {}", error.diagnostic_code),
    )
}
