use std::{env, fs};
use nape_kernel::error::{Error, Kind};
use nape_kernel::values::directory::directory_list::DirectoryList;

/// # Overview
///
/// An implementation of [`DirectoryCreationGateway`] using the [`fs`] (file system) implementation of the ['std'] rust library.
///
/// # Design Decisions
///  * This implementation assess what the current directory is and then creates the directories in the [`DirectoryList`] in the current directory.
///  * The return [`DirectoryList`] contains the fully qualified paths of the directories created and the names of the directories provided in the `directory_list` argument.
///
pub fn create_directories_on_filesystem(directory_list: &DirectoryList) -> Result<DirectoryList, Error> {

    let mut updated_directory_list = DirectoryList::default();
    let current_dir = env::current_dir()
        .map_err(|err| { Error::for_system(Kind::GatewayError,
                                           format!("Failed to get current directory. {}", err))
    })?;

    directory_list.paths.iter()
        .map(|(path_name, directory_path)| {

            let fully_qualified_directory_path = &current_dir.join(directory_path);

            fs::create_dir_all(fully_qualified_directory_path)
                .map_err(|err| {
                    Error::for_system(Kind::GatewayError,
                                      format!("Failed to create directory '{}'. {}", directory_path, err))
                })?;

            let fully_qualified_directory_path_as_str = match fully_qualified_directory_path.to_str() {
                Some(path) => path,
                None => {
                    return Err(Error::for_system(Kind::GatewayError,
                                                 format!("Failed to convert path to string. '{}'", fully_qualified_directory_path.display())))
                } };

            updated_directory_list = updated_directory_list.try_add(
                path_name,
                fully_qualified_directory_path_as_str)
                .map_err(|err| { Error::for_system(Kind::GatewayError,
                                                   format!("Failed to add path ['{}', '{}'] to directory list. '{}'", path_name, fully_qualified_directory_path_as_str, err.message))
                })?;

            Ok(())

        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(updated_directory_list)

}