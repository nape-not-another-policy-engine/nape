
use crate::error::{Audience, Kind};
use crate::values::specification::repository_link::RepositoryLink;
use crate::values::specification::repository_link::RepositoryLinkScheme;


mod repository_link_tests {
    use super::*;
    #[test]
    fn new_repository_link_without_scheme_in_url() {
        let process_link_result = RepositoryLink::try_new("github.com/nape/processes.git");
        match process_link_result {
            Ok(process_link) => {
                assert_eq!(process_link.value, "https://github.com/nape/processes.git");
                assert_eq!(process_link.url.scheme, "https");
                assert_eq!(process_link.url.host, "github.com");
                assert_eq!(process_link.url.port, 0);
                assert_eq!(process_link.url.path, "/nape/processes.git");
                assert_eq!(process_link.url.query_string, "");
                assert_eq!(process_link.url.queries.len(), 0);
                assert_eq!(process_link.url.fragment, "");
            },
            Err(error) => { assert!(false, "An error was returned.  Did not expect an error: [{0}]", error.message) }
        }
    }
    #[test]
    fn new_repository_link_with_scheme_in_url() {
        let process_link_result = RepositoryLink::try_new("https://github.com/nape/processes/rust-ci");
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
            Err(error) => { assert!(false, "An error was returned.  Did not expect an error: [{0}]", error.message) }
        }
    }
    #[test]
    fn is_scheme_success() {
        let process_link = RepositoryLink::try_new("local://some/repo").unwrap();

        assert!(process_link.is_scheme(&RepositoryLinkScheme::Local));
    }
    #[test]
    fn new_repository_link_error_empty_url() {
        let result = RepositoryLink::try_new("");
        match result {
            Ok(_) => {
                assert!(false, "Expected an error, but did not receive an error");
            },
            Err(error) => {
                assert_eq!(error.kind, Kind::InvalidInput);
                assert_eq!(error.audience, Audience::User);
                assert_eq!(error.message, "You provided an empty url, please provide a valid url for the procedure link.");
            }
        }
    }
    #[test]
    fn new_repository_link_error_empty_url_of_whitespaces() {
        let result = RepositoryLink::try_new(" ");
        match result {
            Ok(_) => {
                assert!(false, "Expected an error, but did not receive an error");
            },
            Err(error) => {
                assert_eq!(error.kind, Kind::InvalidInput);
                assert_eq!(error.audience, Audience::User);
                assert_eq!(error.message, "You provided an empty url, please provide a valid url for the procedure link.");
            }
        }
    }
    #[test]
    fn new_repository_link_error_allowed_scheme() {
        let result = RepositoryLink::try_new("not-supported://localhost");

        assert!(result.is_err(), "{}", format!("Expected an error, but did not receive an error. {:?}",result.unwrap()));

        let error = result.err().unwrap();
        assert_eq!(error.kind, Kind::InvalidInput);
        assert_eq!(error.audience, Audience::User);
        assert!(error.message.contains("The scheme 'not-supported' is not allowed. Allowed schemes are "));

    }
    #[test]
    fn is_scheme_error() {
        let process_link = RepositoryLink::try_new("local://some/repo").unwrap();

        assert_eq!(false, process_link.is_scheme(&RepositoryLinkScheme::Https));

    }
}

mod repository_link_scheme_tests {
    use super::*;

    #[test]
    fn default_scheme_success() {
        let default_scheme = RepositoryLinkScheme::default();
        assert_eq!(default_scheme, RepositoryLinkScheme::Https);
    }

    #[test]
    fn is_allowed_schemes_success() {
        let is_allowed = RepositoryLinkScheme::is_allowed("https");
        assert!(is_allowed);

        let is_allowed = RepositoryLinkScheme::is_allowed("local");
        assert!(is_allowed);

        let is_allowed = RepositoryLinkScheme::is_allowed("ssh");
        assert!(is_allowed);

        let is_allowed = RepositoryLinkScheme::is_allowed("nape");
        assert!(is_allowed);
    }

    #[test]
    fn allowed_schemes_success() {
        let allowed_schemes = RepositoryLinkScheme::allowed();
        assert_eq!(allowed_schemes.len(), 4);
        assert!(allowed_schemes.contains(&"https"));
        assert!(allowed_schemes.contains(&"nape"));
        assert!(allowed_schemes.contains(&"local"));
        assert!(allowed_schemes.contains(&"ssh"));
    }

    #[test]
    fn as_str_success() {
        let scheme = RepositoryLinkScheme::Https;
        assert_eq!(scheme.as_str(), "https");

        let scheme = RepositoryLinkScheme::Nape;
        assert_eq!(scheme.as_str(), "nape");

        let scheme = RepositoryLinkScheme::Local;
        assert_eq!(scheme.as_str(), "local");

        let scheme = RepositoryLinkScheme::Ssh;
        assert_eq!(scheme.as_str(), "ssh");
    }

    #[test]
    fn is_allowed_scheme_error() {
        let is_allowed = RepositoryLinkScheme::is_allowed("wrong-scheme");
        assert!(!is_allowed);
    }

}
