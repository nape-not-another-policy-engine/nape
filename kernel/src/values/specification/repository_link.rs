use crate::error::{Error, Kind};
use crate::values::uri::url::URL;

/// Enum representing allowed repository link schemes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryLinkScheme {
    Https,
    Local,
    Nape,
    Ssh,
}

impl RepositoryLinkScheme {

    /// Return the default scheme of [`RepositoryLinkScheme::Https`]
    pub fn default() -> RepositoryLinkScheme {
        RepositoryLinkScheme::Https
    }

    /// Convert the enum variant to its string representation.
    pub fn as_str(&self) -> &str {
        match self {
            RepositoryLinkScheme::Https => "https",
            RepositoryLinkScheme::Local => "local",
            RepositoryLinkScheme::Nape => "nape",
            RepositoryLinkScheme::Ssh => "ssh",
        }
    }

    /// Check if a given scheme is allowed.
    pub fn is_allowed(scheme: &str) -> bool {
        matches!(scheme, "https" | "nape"  | "local"  | "ssh")
    }

    /// Return a list of allowed schemes.
    pub fn allowed() -> Vec<&'static str> {
        ["https", "nape", "local", "ssh"].to_vec()
    }

}


/// The [`RepositoryLink`] value is an NAPE-specific value for capturing the URL for the location of a procedure specification.
///
/// # Assumptions
///  * This defaults all schemes to **https://** if a scheme is not provided.
///  * This allows url inputs values such as **localhost** to be valid.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RepositoryLink {
    pub url: URL,
    pub value: String
}

impl RepositoryLink {

    /// Create a new [`RepositoryLink`] value from a url string.
    ///
    /// # Arguments
    ///
    /// * `value` - A string that represents the URL of the procedure specification.
    ///
    /// # Returns
    ///
    /// A [`RepositoryLink`] if the URL is valid, otherwise an [`Error`] of kind [`Kind::InvalidInput`] for audience [`Audience::User`].
    ///
    pub fn try_new(value: &str) -> Result<RepositoryLink, Error> {
        check_for_empty(value)?;
        let cleaned_value = apply_default_scheme(value);
        let scheme_approved_value = validate_scheme(&cleaned_value)?;
        match URL::new(&scheme_approved_value) {
            Ok(url) => Ok(RepositoryLink { url, value: cleaned_value.to_string()}),
            Err(error) => Err(Error::for_user(Kind::InvalidInput,
                format!("The procedure link is not valid - {}", error.message)))
        }
    }
}

fn check_for_empty(url: &str) -> Result<(), Error> {
    if url.trim().is_empty() {
        return Err(
            Error::for_user(
                Kind::InvalidInput, "You provided an empty url, please provide a valid url for the procedure link.".to_string()))
    }
    Ok(())
}

/// Apply the default scheme of [`RepositoryLinkScheme`] to the URL if the URL does not have a scheme.
fn apply_default_scheme(url: &str) -> String {
    let url_without_scheme = url.split("://").collect::<Vec<&str>>();
    if url_without_scheme.len() > 1 { return url.to_string(); }
    format!("{}://{}", RepositoryLinkScheme::default().as_str(), url.trim())
}

fn validate_scheme(url: &str) -> Result<String, Error> {
    let url_parts: Vec<&str> = url.split("://").collect();
    let scheme = url_parts[0];
    if RepositoryLinkScheme::is_allowed(&scheme) {
        Ok(url.to_string())
    } else {
        Err(Error::for_user(
            Kind::InvalidInput,
            format!("The scheme '{}' is not allowed. Allowed schemes are {:?}", scheme, RepositoryLinkScheme::allowed())
        ))
    }
}