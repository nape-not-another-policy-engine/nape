use crate::error::{Error, Kind};
use crate::values::specification::repository_link::RepositoryLink;

/// # Overview
///
/// A struct that represents a Procedure.
///
/// # Attributes
///
/// * `repository` - A string slice representing the repository link where the [`Procedure`] data is stored
/// * `directory` - A string slice representing the directory path of the where the [`Procedure`] information is located within the repository.
///
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Procedure {
    pub repository: String,
    pub directory: String,
}

impl Procedure {

    /// # Overview
    ///
    /// Attempts to create a new Procedure instance.
    ///
    /// # Arguments
    ///
    /// * `repository` - A string slice representing the repository link of the Procedure.
    /// * `directory` - A string slice representing the directory path of the Procedure.
    ///
    /// # Returns
    ///
    /// - A [`Result`] containing either a [`Procedure`] instance or an [`Error`].
    ///  - All returned errors are for the audience [`Audience::User`], and of kind [`Kind::InvalidInput`].
    ///
    pub fn try_new(repository: &str, directory: &str) -> Result<Procedure, Error> {
        let valid_repo = validate_repository_link(repository)?;
        let valid_directory = validate_directory(directory)?;
        Ok( Procedure { repository: valid_repo.value.clone(), directory: valid_directory } )
    }
}

fn validate_repository_link(link: &str) -> Result<RepositoryLink, Error> {
    RepositoryLink::new(link).map_err(|e|
        Error::for_user( Kind::InvalidInput, format!("'{}' is not a valid procedure location: {}", link, e.message)))
}

fn validate_directory(directory: &str) -> Result<String, Error> {
    let mut previous_char = None;

    if directory.is_empty() || directory == "/" {
        return Ok(String::from("/"))
    }

    for (i, c) in directory.chars().enumerate() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => (),
            '/' => {
                if let Some('/') = previous_char {
                    return Err(Error::for_user(Kind::InvalidInput, format!("The directory path cannot contain contiguous forward slashes at position {}. Please remove the extra forward slash.", i)));
                }
                if i == 0 || i == directory.len() - 1 {
                    return Err(Error::for_user(Kind::InvalidInput, "The directory path cannot start or end with a forward slash at position.".to_string()));
                }
            }
            _ => return Err(Error::for_user(Kind::InvalidInput, format!("Invalid character at position {}: '{}'. The directory path can only contain alphanumeric characters, underscores, and non-contiguous forward slashes.", i, c))),
        }
        if (i == 0 || i == directory.len() - 1) && c == '-' {
            return Err(Error::for_user(Kind::InvalidInput, "The directory path cannot start or end with a dash.".to_string()));
        }
        previous_char = Some(c);
    }

    Ok(String::from(directory))

}
