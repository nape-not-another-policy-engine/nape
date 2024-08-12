use nape_testing_assertions::kernel_error_starts_with;
use crate::algorithms::signature_algorithm::Signature;
use crate::algorithms::signature_algorithm::SignatureType::SHA256;
use crate::error::{Audience, Kind};
use crate::values::specification::assurnace_report::action::Action;
use crate::values::specification::outcome::Outcome;
use crate::values::specification::assurnace_report::signed_file::SignedFile;
use crate::values::specification::description::Description;
use crate::values::specification::name::Name;


#[test]
fn builder_use_only_success() {
    let test_file_sig = Signature::try_new(SHA256, "the-test-file-signature").unwrap();
    let test_file = SignedFile::new("./some-location/file.txt", &test_file_sig).unwrap();
    let evidence_file_sig = Signature::try_new(SHA256, "the-evidence-file-signature").unwrap();
    let evidence_file = SignedFile::new("./some-location/file.txt", &evidence_file_sig).unwrap();

    let action = Action::builder()
        .use_name(&Name::try_from("action-name").unwrap())
        .use_outcome(&Outcome::PASS)
        .use_reason(&Description::try_from("action reason").unwrap())
        .use_test_file_signature(&test_file)
        .use_evidence_file_signature(&evidence_file)
        .try_build().unwrap();

    assert_eq!(action.name(), &Name::try_from("action-name").unwrap());
    assert_eq!(action.outcome(), &Outcome::PASS);
    assert_eq!(action.reason(), &Description::try_from("action reason").unwrap());
    assert_eq!(action.test_file(), &test_file);
    assert_eq!(action.evidence_file(), &evidence_file);
}

#[test]
fn builder_str_only_success() {

    let action = Action::builder()
        .name("action-name")
        .outcome("pass")
        .reason("action reason")
        .test_file_path("./some-test/file.txt")
        .test_file_signature("SHA256[testsignature]")
        .evidence_file_path("./some-evidence/file.txt")
        .evidence_file_signature("SHA256[evidencesignature]")
        .try_build().unwrap();

    assert_eq!(action.name(), &Name::try_from("action-name").unwrap());
    assert_eq!(action.outcome(), &Outcome::PASS);
    assert_eq!(action.reason(), &Description::try_from("action reason").unwrap());

    let test_file_sig = Signature::try_new(SHA256, "testsignature").unwrap();
    let test_file = SignedFile::new("./some-test/file.txt", &test_file_sig).unwrap();
    let evidence_file_sig = Signature::try_new(SHA256, "evidencesignature").unwrap();
    let evidence_file = SignedFile::new("./some-evidence/file.txt", &evidence_file_sig).unwrap();

    assert_eq!(action.test_file(), &test_file);
    assert_eq!(action.evidence_file(), &evidence_file);
}

#[test]
fn builder_ensure_use_overrides_str_success() {
    let test_file_sig = Signature::try_new(SHA256, "the-test-file-signature").unwrap();
    let test_file = SignedFile::new("./some-location/file.txt", &test_file_sig).unwrap();
    let evidence_file_sig = Signature::try_new(SHA256, "the-evidence-file-signature").unwrap();
    let evidence_file = SignedFile::new("./some-location/file.txt", &evidence_file_sig).unwrap();

    let action = Action::builder()
        .use_name(&Name::try_from("action-name").unwrap())
        .use_outcome(&Outcome::PASS)
        .use_reason(&Description::try_from("action reason").unwrap())
        .use_test_file_signature(&test_file)
        .use_evidence_file_signature(&evidence_file)
        .name("name-shoud-not-appear")
        .outcome("fail")
        .reason("bad reason")
        .test_file_path("./some-bad-test/file.txt")
        .test_file_signature("SHA256[testsignature]")
        .evidence_file_path("./some-bad-evidence/file.txt")
        .evidence_file_signature("SHA256[evidencesignature]")
        .try_build().unwrap();

    assert_eq!(action.name(), &Name::try_from("action-name").unwrap());
    assert_eq!(action.outcome(), &Outcome::PASS);
    assert_eq!(action.reason(), &Description::try_from("action reason").unwrap());
    assert_eq!(action.test_file(), &test_file);
    assert_eq!(action.evidence_file(), &evidence_file);

}

#[test]
fn builder_invalid_name_error() {
    let test_file_sig = Signature::try_new(SHA256, "the-test-file-signature").unwrap();
    let test_file = SignedFile::new("./some-location/file.txt", &test_file_sig).unwrap();
    let evidence_file_sig = Signature::try_new(SHA256, "the-evidence-file-signature").unwrap();
    let evidence_file = SignedFile::new("./some-location/file.txt", &evidence_file_sig).unwrap();

    let result = Action::builder()
        .name("action name")
        .use_outcome(&Outcome::PASS)
        .reason("action reason")
        .use_test_file_signature(&test_file)
        .use_evidence_file_signature(&evidence_file)
        .try_build();

    kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "There is an issue with the name 'action name'. ");

}

#[test]
fn builder_invalid_outcome_error() {
    let result = Action::builder()
        .name("action-name")
        .outcome("not-an-outcome")
        .reason("action reason")
        .test_file_path("./some-test/file.txt")
        .test_file_signature("SHA256[testsignature]")
        .evidence_file_path("./some-evidence/file.txt")
        .evidence_file_signature("SHA256[evidencesignature]")
        .try_build();

    kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "There is an issue with the outcome 'not-an-outcome'. ");
}

#[test]
fn  builder_invalid_reason_error() {
    let test_file_sig = Signature::try_new(SHA256, "the-test-file-signature").unwrap();
    let test_file = SignedFile::new("./some-location/file.txt", &test_file_sig).unwrap();
    let evidence_file_sig = Signature::try_new(SHA256, "the-evidence-file-signature").unwrap();
    let evidence_file = SignedFile::new("./some-location/file.txt", &evidence_file_sig).unwrap();

    let result = Action::builder()
        .name("action-name")
        .use_outcome(&Outcome::PASS)
        .reason("  ")
        .use_test_file_signature(&test_file)
        .use_evidence_file_signature(&evidence_file)
        .try_build();

    kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "There is an issue with the reason '  '. ");

}

#[test]
fn builder_invalid_test_file_path_error() {

    let result = Action::builder()
        .name("action-name")
        .outcome("pass")
        .reason("action reason")
        .test_file_path("")
        .test_file_signature("SHA256[testsignature]")
        .evidence_file_path("./some-evidence/file.txt")
        .evidence_file_signature("SHA256[evidencesignature]")
        .try_build();

    kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "There is an issue with the test file path ''. ");
}

#[test]
fn builder_invalid_test_file_signature_error() {

    let result = Action::builder()
        .name("action-name")
        .outcome("pass")
        .reason("action reason")
        .test_file_path("./some-test/file.txt")
        .test_file_signature("BILL[testsignature]")
        .evidence_file_path("./some-evidence/file.txt")
        .evidence_file_signature("SHA256[evidencesignature]")
        .try_build();

    kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "There is an issue with the test file signature 'BILL[testsignature]'. ");
}

#[test]
fn builder_invalid_evidence_file_path_error() {

    let result = Action::builder()
        .name("action-name")
        .outcome("pass")
        .reason("action reason")
        .test_file_path("./some-test/file.txt")
        .test_file_signature("SHA256[testsignature]")
        .evidence_file_path("")
        .evidence_file_signature("SHA256[evidencesignature]")
        .try_build();

    kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "There is an issue with the evidence file path ''. ");
}

#[test]
fn builder_invalid_evidence_file_signature_error() {

    let result = Action::builder()
        .name("action-name")
        .outcome("pass")
        .reason("action reason")
        .test_file_path("./some-test/file.txt")
        .test_file_signature("SHA256[testsignature]")
        .evidence_file_path("./some-evidence/file.txt")
        .evidence_file_signature("BILL[evidencesignature]")
        .try_build();

    kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "There is an issue with the evidence file signature 'BILL[evidencesignature]'. ");
}