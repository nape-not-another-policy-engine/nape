
use crate::error::{Audience, Kind};
use crate::values::specification::repository_link::RepositoryLink;

#[test]
fn new_repository_link_without_scheme_in_url() {
    let process_link_result = RepositoryLink::new("github.com/nape/processes/rust-ci");
    match process_link_result {
        Ok(process_link) => {
            assert_eq!(process_link.value, "git://github.com/nape/processes/rust-ci");
            assert_eq!(process_link.url.scheme, "git");
            assert_eq!(process_link.url.host, "github.com");
            assert_eq!(process_link.url.port, 0);
            assert_eq!(process_link.url.path, "/nape/processes/rust-ci");
            assert_eq!(process_link.url.query_string, "");
            assert_eq!(process_link.url.queries.len(), 0);
            assert_eq!(process_link.url.fragment, "");
        },
        Err(error) => { assert!(false, "An error was returned.  Did not expect an error: [{0}]", error.message )}
    }
}
#[test]
fn new_repository_link_with_scheme_in_url() {
    let process_link_result = RepositoryLink::new("https://github.com/nape/processes/rust-ci");
    match process_link_result {
        Ok(process_link) => {
            assert_eq!(process_link.value, "https://github.com/nape/processes/rust-ci");
            assert_eq!(process_link.url.scheme, "https");
            assert_eq!(process_link.url.host, "github.com");
            assert_eq!(process_link.url.port, 0);
            assert_eq!(process_link.url.path, "/nape/processes/rust-ci");
            assert_eq!(process_link.url.query_string, "");
            assert_eq!(process_link.url.queries.len(), 0);
            assert_eq!(process_link.url.fragment, "");
        },
        Err(error) => { assert!(false, "An error was returned.  Did not expect an error: [{0}]", error.message )}
    }
}
#[test]
fn new_repository_link_error_empty_url() {
    let result = RepositoryLink::new("");
    match result {
        Ok(_) => {
            assert!(false, "Expected an error, but did not receive an error");
        },
        Err(error) => {
            assert_eq!(error.kind, Kind::InvalidInput);
            assert_eq!(error.audience, Audience::User);
            assert_eq!(error.message, "You provided an empty url,  please provide a valid url for the procedure link.");
        }
    }
}
#[test]
fn new_repository_link_error_empty_url_of_whitespaces() {
    let result = RepositoryLink::new(" ");
    match result {
        Ok(_) => {
            assert!(false, "Expected an error, but did not receive an error");
        },
        Err(error) => {
            assert_eq!(error.kind, Kind::InvalidInput);
            assert_eq!(error.audience, Audience::User);
            assert_eq!(error.message, "You provided an empty url,  please provide a valid url for the procedure link.");
        }
    }
}
#[test]
fn new_repository_link_error_allowed_scheme() {
    let result = RepositoryLink::new("ssh://localhost");

    assert!(result.is_err(), "{}", format!("Expected an error, but did not receive an error. {:?}",result.unwrap()));

    let error = result.err().unwrap();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert!(error.message.contains("The scheme 'ssh' is not allowed. Allowed schemes are "));

}

