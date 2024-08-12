
use crate::error::{Audience, Kind};
use crate::values::nrn::nrn::{NRN, NapeNID, NID, NSS};

/*** NRN Tests ***/

#[test]
fn nrn_from_str_success() {
    let result = NRN::new("nrn:sourcecode:nape:project/collection-cli");

    assert!(result.is_ok());

    let nrn = result.unwrap();
    assert_eq!(nrn.scheme, "nrn");
    assert_eq!(nrn.nid, NapeNID::SourceCode);
    assert_eq!(nrn.nss[0], NSS::new("nape").unwrap());
    assert_eq!(nrn.nss[1], NSS::new("project/collection-cli").unwrap());
}
#[test]
fn nrn_from_str_error_empty() {
    let result = NRN::new("");

    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.message, "You provided an empty NRN value. An NRN must be in the format 'nrn:<nid>:<nss>'");
}
#[test]
fn nrn_from_str_error_wrong_scheme() {
    let result = NRN::new("urn:sourecode:nape::project/collection-cli");

    assert!(result.is_err());
    assert_eq!(result.err().unwrap().message, "You provided 'urn:sourecode:nape::project/collection-cli' as an NRN and the scheme 'urn'  is not valid. Must be 'nrn'");
}
#[test]
fn nrn_from_str_error_wrong_nid() {
    let result = NRN::new("nrn:somewrongnid:nape::project/collection-cli");

    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.message, "'somewrongnid' is not a valid NID. Must be one of: [procedure, sourcecode].");
}
#[test]
fn nrn_from_str_error_missing_nss() {
    let result = NRN::new("nrn:sourcecode");
    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.message, "The supplied NRN 'nrn:sourcecode' must have at least one NSS after the NID.");
}
#[test]
fn nrn_from_str_error_invalid_nss() {
    let result = NRN::new("nrn:sourcecode:nape:project/collection-cli:invalid nss");
    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.message, "Input 'invalid nss' contains whitespace character ' ' at position 7");
}


/** NAPE NID Tests ***/

#[test]
fn nid_display() {
    let result = format!("{}", NapeNID::SourceCode);
    assert_eq!(result, "sourcecode");
    assert_eq!(result.to_string(), "sourcecode");
}
#[test]
fn nid_sourcecode_display() {
    let result = format!("{}", NapeNID::Procedure);
    assert_eq!(result, "procedure");
    assert_eq!(result.to_string(), "procedure");
}

#[test]
fn nid_does_not_exist() {
    let nid = NID::new("does-not-exist").unwrap();
    let does_not_exist = NapeNID::new(nid);

    assert!(does_not_exist.is_err());
    let dne_error = does_not_exist.err().unwrap();
    assert_eq!(dne_error.kind, Kind::InvalidInput);
    assert_eq!(dne_error.audience, Audience::User);
    assert_eq!(dne_error.message, "'does-not-exist' is not a valid NID. Must be one of: [procedure, sourcecode].");
}
#[test]
fn verify_nid_exists()  {

    let sourcecode_nid = NapeNID::new(NID::new("sourcecode").unwrap());
    let procedure_nid = NapeNID::new(NID::new("procedure").unwrap());


    assert!(sourcecode_nid.is_ok());
    assert_eq!(sourcecode_nid.unwrap(), NapeNID::SourceCode);
    ("procedure");

    assert!(procedure_nid.is_ok());
    assert_eq!(procedure_nid.unwrap(), NapeNID::Procedure);

}

/*** NID Tests ***/

#[test]
fn new_nid_success() {
    let result = NID::new("sourcecode");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, "sourcecode");
}
#[test]
fn new_nid_error_empty() {
    let result = NID::new("");
    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.message, "The NID provided is blank.");
}
#[test]
fn new_nid_error_first_char_not_alphabetical() {
    let result = NID::new("1sourcecode");
    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.message, "The NID must start with an alphabetic character. You NID '1sourcecode' starts with '1'.");
}
#[test]
fn new_nid_error_invalid_character() {
    let result = NID::new("source code");
    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.message, "NID 'source code' contains invalid character ' ' at position 6.  An NID can only contains alphanumeric characters or a dash '-'.")
}
#[test]
fn new_nid_greater_than_31_characters() {
    let result = NID::new("sourcecode1234567890123456789012");
    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.message, "The NID 'sourcecode1234567890123456789012' is too long. An NID must be 31 characters or less.")
}


/*** NSS Tests ***/

#[test]
fn test_nss_new() {
    let result = NSS::new("test");
    assert!(result.is_ok());
}
#[test]
fn test_nss_new_with_whitespace() {
    let result = NSS::new("test test");
    assert!( result.is_err() );
    assert_eq!(result.err().unwrap().message, "Input 'test test' contains whitespace character ' ' at position 4");
}
#[test]
fn test_nss_new_with_non_ascii() {
    let result = NSS::new("test😀");
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().message, "Input 'test😀' contains non-ASCII character '😀' at position 4");
}