
use crate::error::{Audience, Kind};
use crate::values::uri::url::URL;

#[test]
fn create_new_url() {
    let url_result = URL::new("http://www.example.com:8080/some/path?query=value#the-fragment");
    match url_result {
        Ok(url) => {
            assert_eq!(url.value(), "http://www.example.com:8080/some/path?query=value#the-fragment");
            assert_eq!(url.scheme, "http");
            assert_eq!(url.host, "www.example.com");
            assert_eq!(url.port, 8080);
            assert_eq!(url.path, "/some/path");
            assert_eq!(url.query_string, "query=value");
            assert_eq!(url.query_count(), 1);
            assert_eq!(url.fragment, "the-fragment")
        }
        Err(error) => { assert!(false, "An error was returned.  Did not expect an error: [{0}]", error.message )}
    }
}

#[test]
fn create_new_url_only_scheme_and_host() {
    let url_result = URL::new("https://www.example.com");
    match url_result {
        Ok(url) => {
            assert_eq!(url.value(), "https://www.example.com");
            assert_eq!(url.scheme, "https");
            assert_eq!(url.host, "www.example.com");
            assert_eq!(url.port, 0);
            assert_eq!(url.path, "/");
            assert_eq!(url.query_string, "");
            assert_eq!(url.query_count(), 0);
            assert_eq!(url.fragment, "")
        }
        Err(error) => { assert!(false, "An error was returned.  Did not expect an error: [{0}]", error.message )}
    }
}

#[test]
fn create_new_url_multiple_queries() {
    let url_result = URL::new("http://www.example.com:8080/some/path?query1=value1&query2=value2#fragment");
    match url_result {
        Ok(url) => {
            assert_eq!(url.value(), "http://www.example.com:8080/some/path?query1=value1&query2=value2#fragment");
            assert_eq!(url.scheme, "http");
            assert_eq!(url.host, "www.example.com");
            assert_eq!(url.port, 8080);
            assert_eq!(url.path, "/some/path");
            assert_eq!(url.query_string, "query1=value1&query2=value2");
            assert_eq!(url.query_count(), 2);
            assert_eq!(true, url.queries.contains(&("query1".to_string(),"value1".to_string())));
            assert_eq!(true, url.queries.contains(&("query2".to_string(),"value2".to_string())));
            assert_eq!(url.fragment, "fragment")
        }
        Err(error) => { assert!(false, "An error was returned.  Did not expect an error: [{0}]", error.message )}
    }
}

#[test]
fn create_from_bad_url_input() {
    let url_result = URL::new("Some-Bad-Input");
    match url_result {
        Ok(_url) => {
            assert!(false, "Was expecting an error, but the URL::new(...) processed successfully.");
        }
        Err(error) => {
            assert_eq!(error.kind, Kind::InvalidInput);
            assert_eq!(error.audience, Audience::User);
            assert!(error.message.starts_with("You provided an invalid Url. "));
            assert!(error.message.contains("Your Input: "));
            assert!(error.message.contains("The Issue: "));
        }
    }
}