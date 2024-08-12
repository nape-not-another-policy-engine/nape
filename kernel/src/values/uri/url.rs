use url::{Host, Url};
use crate::error::{Error, Kind};
use crate::values::text::line::Line;

/// The URL value is an NAPE-specific value for capturing the RFC {SPEC HERE}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct URL {
    pub value: Line,
    pub scheme: String,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub query_string: String,
    pub queries: Vec<(String, String)>,
    pub fragment: String
}

impl URL {
    pub fn new(url: &str) -> Result<Self, Error> {
        let raw_input = Line::new(url);
        Url::parse(&raw_input.value())
            .map_err(|error| {
                Error::for_user(
                    Kind::InvalidInput,
                    format!("You provided an invalid Url. \
                            Your Input: [{0}], The Issue: [{1}]",
                            raw_input.value(),
                            error.to_string()))
            })
            .and_then(|parsed_url| {
                Ok(URL {
                    value: raw_input,
                    scheme: parsed_url.scheme().to_string(),
                    host: parsed_url.host().unwrap_or(Host::Domain("")).to_string(),
                    port: parsed_url.port().unwrap_or(0),
                    path: parsed_url.path().to_string(),
                    query_string: parsed_url.query().unwrap_or("").to_string(),
                    queries: extract_multiple_queries(&parsed_url),
                    fragment: parsed_url.fragment().unwrap_or("").to_string(),
                })
            })
    }
    pub fn value(&self) -> String {
        self.value.value()
    }
    pub fn queries(&self) -> Vec<(String, String)> { self.queries.clone() }
    pub fn query_count(&self) -> usize { self.queries.len() }
}

fn extract_multiple_queries(parsed_url: &Url) -> Vec<(String, String)> {
    let mut queries: Vec<(String, String)> = Vec::new();
    for (key, value) in parsed_url.query_pairs() {
        queries.push((key.into_owned(), value.into_owned()));
    }
    return queries;
}