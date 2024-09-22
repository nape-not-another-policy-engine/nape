use nape_kernel::algorithms::signature_algorithm::SignatureType;
use nape_kernel::error::{Audience, Kind};
use nape_testing_assertions::{is_ok, kernel_error_eq};
use crate::gateway_adapter::sha2::signature_algorithm::sha256_signature;

#[test]
fn success() {

    let data = vec![1, 2, 3, 4, 5];
    let result = sha256_signature(&data);

    is_ok!(&result);

    let signature = result.unwrap();
    assert_eq!(signature.signature_type(), &SignatureType::SHA256);
    assert_eq!(signature.to_string(), "74f81fe167d99b4cb41d6d0ccda82278caee9f3e2f25d5e5a3936ff3dcec60d0");

}

#[test]
fn no_data_error() {
    let data = vec![];
    let result = sha256_signature(&data);

    kernel_error_eq!(&result,
        Kind::InvalidInput,
        Audience::System,
        "Failed to generate a SHA256 signature because the input data you provided is empty.");
}