use std::collections::HashMap;
use crate::error::{Error, Kind};

/// The [`DirectoryList`]  is a list of paths to directories or files which is used to model an expected directory structure, with files, on a file system which will store the evidence, specification, and other files.  This is not specific to any particular file system, but is a general model of a directory structure.
///
///
/// **Any function using the DirectoryList as a model is expected to implement their own OS specific implementations of these file paths.**
///
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct DirectoryList {
    pub paths: Vec<(String, String)>
}

impl DirectoryList {

    /// Creates a new [`DirectoryList`] from an existing Vec<(String,String)> which contains paths to directories or files.
    ///
    /// # Arguments
    ///
    /// * `paths` - A vector of paths to directories or files.
    ///
    /// # Returns
    ///
    /// A [`DirectoryList`] struct or an [`Error`].
    ///
    /// # Errors
    ///
    /// If the vector of **paths** is empty, an [`Error`] is returned for the [`Audience::System`] with [`Kind::InvalidInput`] and a message indicating that no directory or file paths have been provided.
    ///
    pub fn try_from_vec(paths: Vec<(String, String)>) -> Result<DirectoryList, Error> {
        if paths.len() == 0 {
            return Err(Error::for_system(Kind::InvalidInput, "No directory or file paths have been provided.".to_string()));
        }
        Ok(DirectoryList{ paths })
    }

    pub fn try_from_hashmap(map: HashMap<String, String>) -> Result<DirectoryList, Error> {
        let mut directory_list = DirectoryList::default();

        for (path_name, path) in map {
            directory_list = directory_list.try_add(&path_name, &path)?;
        }

        Ok(directory_list)
    }

    /// Adds a path to the `[DirectoryList`].
    ///
    /// # Arguments
    ///
    /// * `path_name` - A string representing a name for the path.
    /// * `path` - A string representing a path to a directory or file.
    ///
    /// # Returns
    ///
    /// A [`DirectoryList`] with the new path added, or an [`Error`].
    ///
    /// # Errors
    ///
    /// - If the **path name** is an empty string, an [`Error`] is returned for the [`Audience::System`] with [`Kind::InvalidInput`] and a message indicating that an empty path name was provided.
    /// - If the **path** is an empty string, an [`Error`] is returned for the [`Audience::System`] with [`Kind::InvalidInput`] and a message indicating that an empty path was provided.
    /// - If the **path name** already exists in the [`DirectoryList`], an [`Error`] is returned for the [`Audience::System`] with [`Kind::InvalidInput`] and a message indicating that the path name already exists in the Directory List.
    ///
    pub fn try_add(&self, path_name: &str, path: &str) -> Result<DirectoryList, Error> {
        if path_name.len() == 0 {
            return Err(Error::for_system(Kind::InvalidInput, "You provided an empty path name to add to the Directory List.  Please provide a non-empty path name.".to_string()));
        }

        if path.len() == 0 {
            return Err(Error::for_system(Kind::InvalidInput, "You provided an empty path to add to the Directory List.  Please provide a non-empty path.".to_string()));
        }

        if self.try_get(path_name).is_some() {
            return Err(Error::for_system(
                Kind::InvalidInput,
                format!("The path name '{}' already exists in the Directory List.  Please provide a unique path name.", path_name)));
        }

        let mut current_list = self.paths.clone();
        current_list.push((path_name.to_string(), path.to_string()));
        Ok(DirectoryList { paths: current_list })
    }

    pub fn try_merge(&self, other: &DirectoryList) -> Result<DirectoryList, Error> {
        let mut updated_list = self.paths.clone();
        for (path_name, path) in &other.paths {
            if self.try_get(path_name).is_some() {
                return Err(Error::for_system(
                    Kind::InvalidInput,
                    format!("The path name '{}' already exists in the Directory List.  Please provide a unique path name.", path_name)));
            }
            updated_list.push((path_name.to_string(), path.to_string()));
        }
        Ok(DirectoryList { paths: updated_list })
    }

    /// Retrieves a path from the [`DirectoryList`] based on the path name.'
    ///
    /// # Arguments
    ///
    /// * `path_name` - A string representing the path name to retrieve the path for.
    ///
    /// # Returns
    ///
    /// An [`Option`] containing the path if it exists in the DirectoryList, otherwise [`None`].
    ///
    pub fn try_get(&self, path_name: &str) -> Option<String> {
        self.paths.iter()
            .find(|(name, _)| name == path_name)
            .map(|(_, path)| path.clone())
    }

}