
use crate::algorithms::signature_algorithm::{Signature, SignatureType};
use crate::error::{Kind, Audience};
use nape_testing_assertions::kernel_error_eq;
use nape_testing_assertions::is_ok;


mod signature_type {
    use super::*;

    #[test]
    fn new_signature_type_error_from_str() {
        let sig_type = SignatureType::from("some-invalid-type");

        kernel_error_eq!(sig_type, Kind::InvalidInput, Audience::System, "The signature algorithm 'some-invalid-type' is not supported. The supported algorithms are: [SHA256]");
    }

    #[test]
    fn new_signature_type_sha256_from_str() {
        let sig_type = SignatureType::from("SHA256");
        assert!(sig_type.is_ok());
        assert_eq!(sig_type.unwrap(), SignatureType::SHA256);
    }

    #[test]
    fn sha256_display_success() {
        let sha256 = SignatureType::SHA256;
        assert_eq!(sha256.to_string(), "SHA256");
    }

}


mod signature {
    use super::*;

    #[test]
    fn try_new_success() {
        let sig = Signature::try_new(SignatureType::SHA256, "signature");
        is_ok!(&sig);
        let sig = sig.unwrap();

        assert_eq!(sig.structure_signature(), "SHA256[signature]");
        assert_eq!(sig.signature_type(), &SignatureType::SHA256);
        assert_eq!(sig.to_string(), "signature");

    }

    #[test]
    fn try_new_empty_string_error() {
        let result = Signature::try_new(SignatureType::SHA256, "");
        kernel_error_eq!(result,
            Kind::InvalidInput,
            Audience::System,
            "The signature you provided is empty. Please provide a non-empty signature value.");
    }

    #[test]
    fn try_from_success() {
        let signature_str = "SHA256[234928039042340892]";
        let result = Signature::try_from(signature_str);
        assert!(result.is_ok());
        let signature = result.unwrap();
        assert_eq!(signature.signature_type(), &SignatureType::SHA256);
        assert_eq!(signature.to_string(), "234928039042340892");
    }

    #[test]
    fn try_from_unsupported_algorithm_error() {
        let signature_str = "BILL123[234928039042340892]";
        let result = Signature::try_from(signature_str);
        kernel_error_eq!(result, Kind::InvalidInput, Audience::System, "The signature algorithm 'BILL123' is not supported. The supported algorithms are: [SHA256]");
    }

    #[test]
    fn try_from_no_algorithm_error() {
        let signature_str = "[234928039042340892]";
        let result = Signature::try_from(signature_str);

        kernel_error_eq!(result,
            Kind::InvalidInput,
            Audience::System,
            "The signature string provided does not have a signature algorithm. The signature algorithm are the characters before the first bracket: ALGO[the-signature-data-here].");
    }

    #[test]
    fn try_from_empty_signature_error() {
        let signature_str = "SHA256[]";
        let result = Signature::try_from(signature_str);
        kernel_error_eq!(result,
            Kind::InvalidInput,
            Audience::System,
            "The signature string provided does not have any signature data. Signature data are the characters between the two brackets: ALGO[the-signature-data-here].");
    }

    #[test]
    fn try_from_no_first_bracket_error() {
        let signature_str = "SHA256234928039042340892]";
        let result = Signature::try_from(signature_str);
        kernel_error_eq!(result,
            Kind::InvalidInput,
            Audience::System,
            "The signature 'SHA256234928039042340892]' is not in the correct format, the first bracket is missing. Check the signature you provided to ensure it in the format of ALGO[the-signature-data].");
    }

    #[test]
    fn try_from_no_last_bracket_error() {
        let signature_str = "SHA256[234928039042340892";
        let result = Signature::try_from(signature_str);
        kernel_error_eq!(result,
            Kind::InvalidInput,
            Audience::System,
            "The signature 'SHA256[234928039042340892' is not in the correct format, the last bracket is missing. Check the signature you provided to ensure it in the format of ALGO[the-signature-data].");
    }

    #[test]
    fn try_from_backwards_brackets() {
        let signature_str = "SHA256]234928039042340892[";
        let result = Signature::try_from(signature_str);
        kernel_error_eq!(result,
            Kind::InvalidInput,
            Audience::System,
             "The signature you provided has the closing bracket before the opening bracket. Check the signature you provided to ensure it in the format of ALGO[the-signature-data].");
    }

    #[test]
    fn try_from_no_brackets_error() {
        let signature_str = "SHA256234928039042340892";
        let result = Signature::try_from(signature_str);
        kernel_error_eq!(result,
            Kind::InvalidInput,
            Audience::System,
            "The signature 'SHA256234928039042340892' is not in the correct format, the first bracket is missing. Check the signature you provided to ensure it in the format of ALGO[the-signature-data].");
    }


}

