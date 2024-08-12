
use crate::values::nrn::filepath_codec::{decode_from_directory_name, encode_as_directory_name};
use crate::values::nrn::nrn::NRN;

#[test]
fn encode_success() {
    let nrn = NRN::new("nrn:procedure:nape/software-procedures:rust-ci/sourcecode-integration").unwrap();
    let result = encode_as_directory_name(&nrn);
    assert_eq!(result, "nrn_procedure_nape_-_software-procedures_rust-ci_-_sourcecode-integration");
}

#[test]
fn decode_success() {
    let result = decode_from_directory_name("nrn_procedure_nape_-_software-procedures_rust-ci_-_sourcecode-integration").unwrap();
    assert_eq!(result.value, "nrn:procedure:nape/software-procedures:rust-ci/sourcecode-integration");
}

#[test]
fn decode_error_bad_nrn() {
    let result = decode_from_directory_name("sourcecode_example");

    assert!(result.is_err(), "{}", format!("An error was expected but non error was returned: {:?}", result.unwrap()));

    let error = result.unwrap_err();

    assert_eq!(error.kind, crate::error::Kind::InvalidInput);
    assert_eq!(error.audience, crate::error::Audience::User);
    assert!(error.message.starts_with("Could not decode the NRN from the directory name: 'sourcecode_example': "));
}