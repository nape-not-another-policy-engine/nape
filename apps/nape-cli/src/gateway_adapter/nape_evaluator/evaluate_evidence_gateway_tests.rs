use nape_domain::evidence_collection::usecases::evaluate_evidence::gateway_boundary::{EvidenceFilePath, TestFilePath};
use nape_domain::evidence_collection::usecases::evaluate_evidence::gateway_boundary::request::EvaluationFiles;
use nape_kernel::values::specification::outcome::Outcome;
use nape_testing_assertions::is_ok;
use crate::gateway_adapter::nape_evaluator::evaluate_evidence_gateway::{nape_evidence_evaluator};
use nape_testing_filesystem::{canonical_path, create_file};

#[test]
fn success() {
    //
    let evidence_file = create_file!("evaluate_evidence_gateway_nape_eval_success/evidence.json", &generate_author_evidence_file());
    let test_file = create_file!("evaluate_evidence_gateway_nape_eval_success/test.py", &generate_author_test_file());

    let evidence_canonical = canonical_path!(evidence_file);
    let test_canonical = canonical_path!(test_file);

    let evidence_path = EvidenceFilePath::from(&evidence_canonical);
    let test_path = TestFilePath::from(&test_canonical);

    let evaluation_files = EvaluationFiles::default().add(&evidence_path, &test_path);

    let result = nape_evidence_evaluator(&evaluation_files);

    is_ok!(&result);

    let eval_results = result.unwrap();

    let actual_result = eval_results.result_for(&evidence_path, &test_path).unwrap();

    assert_eq!(actual_result.outcome,Outcome::PASS);
    assert_eq!(actual_result.reason.value, "The author has achieved the status of complete.");


}

fn generate_author_evidence_file() -> String {
    r#"
{
  "author": "Bill Bensing",
  "status": "complete"
}
    "#.to_string()
}

fn generate_author_test_file() -> String {
    r#"
import json


def evaluate(evidence_file):
    author_evidence = json.loads(''.join(evidence_file))

    try:
        status = author_evidence['status']
    except KeyError:
        return 'inconclusive', 'The expected data field \'status\' is not present in the evidence file.'
    except Exception as e:
        return 'error', 'An unexpected error occurred while evaluating the evidence. ' + str(e)

    if status is None or status == '':
        return 'inconclusive', 'The expected data field \'status\' does not contain a value.'
    if status == 'complete':
        return 'pass', 'The author has achieved the status of complete.'
    else:
        return 'fail', f'The author has not achieved the status of complete, their current status is \'{status}\'.', ''
        "#.to_string()
}