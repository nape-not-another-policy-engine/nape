use std::{fs};
use std::fs::{OpenOptions, remove_file};
use std::io::Write;
use std::path::Path;
use nape_kernel::error::{Error, Kind};
use nape_kernel::values::specification::file_path::FilePath;

/// An implementation of the [`FileMoveGateway`] using the Rust [`std::fs`] library.
///
/// This implementation does not move the source file or directory, it copies the source to the target and then deletes the source.  This is done to ensure that the source is not lost if the move fails.
///
/// # Arguments
///
/// * `source` - The path of the source file or directory you want to move.
/// * `target` - The path of the target directory were you want to move the source to.
///
///  **Ensure that all source and target arguments have been sanitized prior to calling this function. This function does not sanitize the input.**
///
/// # Returns
///
/// * `Result<(), Error>` - A result containing either an error or nothing.
///
/// # Errors
///
/// All [`Error`] returned by this function are for the [`Audience::System`].
///
/// * If the source does not exist, an error with [`Kind::NotFound`] is returned.
/// * If the target directory does not exist, an error with [`Kind::NotFound`] is returned.
/// * If the source is neither a file nor a directory, an error with [`Kind::InvalidInput`] is returned.
/// * If the file or directory cannot be moved, an error with [`Kind::GatewayError`] is returned.
///
pub fn move_file_on_filesystem(source: &str, target_directory: &str) -> Result<FilePath, Error> {
    let source_path = Path::new(source);
    let target_directory_path = Path::new(target_directory);

    validate_source(source_path)?;
    validate_target_directory(target_directory_path)?;


    return if source_path.is_file() {
        let file_path = copy_file_to_directory(source_path, target_directory_path)?;
        Ok(file_path)
    } else if source_path.is_dir() {
        let file_path = rename_directory(source_path, target_directory_path)?;
        Ok(file_path)
    } else {
        Err(Error::for_system(Kind::InvalidInput, "The source path is neither a file nor a directory.".to_string()))
    }

}

fn validate_target_directory(target_path: &Path) -> Result<(), Error> {

    if !target_path.exists() {
        return Err(Error::for_system(Kind::NotFound,
                                     format!("The target directory your provided '{}' does not exist.  Please create this directory then retry your attempt to move file(s) to it.", target_path.display() ))) }

    if target_path.is_file() {
        return Err(Error::for_system(Kind::InvalidInput,
                                     format!("The target directory you provided '{}' is a file and not a directory.  Please provide a valid directory path as your target directory.", target_path.display())
        ));}

    if !(target_path.is_dir() || target_path.is_file()) {
        return Err(Error::for_system(Kind::InvalidInput,
                                     format!("The target directory you provided '{}' is a null device.  Please provide a valid directory path as your target directory.", target_path.display())
        ));}

    verify_writable_directory(target_path)?;

    Ok(())
}

fn verify_writable_directory(target_path: &Path) -> Result<(), Error> {
    let test_file_path = target_path.join("test_if_directory_is_writable.tmp");
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&test_file_path)
        .map_err(|e| Error::for_system(Kind::PermissionDenied,
                                     format!("The target directory you provided '{}' is not writable. Please ensure you have write access to this directory. {}", target_path.display(),e )))?;
    file.write_all(b"test")
        .map_err(|e| Error::for_system(Kind::PermissionDenied,
                                     format!("We are unable to write to the test file '{}'. This means you may not have write permissions to the target directory. {}", target_path.display(), e)))?;
    remove_file(&test_file_path)
        .map_err( |e| Error::for_system(Kind::GatewayError,
                                     format!("Failed to remove the test file '{}' that is created to test if the target directory is writable.. {}", test_file_path.display(), e)))?;
    Ok(())
}

fn validate_source(source_path: &Path) -> Result<(), Error> {

    if !source_path.exists() {
        return Err(Error::for_system(Kind::NotFound,
                                     format!("The source you provided '{}' does not exist.  This must first exist before you can move it.", source_path.display() )))
    };

    if !(source_path.is_dir() || source_path.is_file()) {
        return Err(Error::for_system(Kind::InvalidInput,
                                     format!("The source you provided '{}' is a null device.  Please provide a valid source path that is either a file or directory.", source_path.display() )))
    };

    if OpenOptions::new().read(true).open(source_path).is_err() {
        return Err(Error::for_system(Kind::PermissionDenied,
                                     format!("The source provided '{}' is not readable. Please ensure you have read access to this source file or directory..", source_path.display())))
    };

    Ok(())

}

/// Copy a file to a target directory.
///
///
/// **Ensure the `source_file` is a file that exists, and the `target_directory` is an existing and accessible directory before calling this function. This function does not validate that source file is a file**
///
/// # Arguments
///
/// * `source_file` - The path of the source file you want to copy.
/// * `target_directory` - The path of the target directory were you want to copy the source file to.
///
fn copy_file_to_directory(source_file: &Path, target_directory: &Path) -> Result<FilePath, Error> {

    // Get the file name to verify the source is a file.
    let file_name = source_file.file_name()
        .ok_or(Error::for_system(Kind::InvalidInput,
                                 format!("The source file '{}' does not have a file name.  Please ensure the source is a file.", source_file.display())))?;

    // Create the new file as the target file
    let target_file = target_directory.join(file_name);

    // Create the target directory if it doesn't exist
    if let Some(parent) = target_file.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|err|
                    Error::for_system(Kind::GatewayError,
                                      format!("Failed to move file '{}'. Failed to create directory '{}'. {}", source_file.display(), parent.display(), err)))?;
        }
    }

    // Copy the file from the source path to the newly created target_file
    fs::copy(source_file, &target_file).map_err(|err| Error::for_system(Kind::GatewayError,
                                         format!("Failed to move file from '{}' to '{}'. {}", source_file.display(), target_file.display(), err) ))?;

    Ok(get_canonical_path(&target_file))

}

fn rename_directory(source_directory: &Path, target_directory: &Path) -> Result<FilePath, Error> {
    fs::rename(source_directory, target_directory)
        .map_err(|err| Error::for_system(Kind::GatewayError,
                                         format!("Failed to move directory from '{}' to '{}'. {}", source_directory.display(), target_directory.display(), err)))?;
    Ok(get_canonical_path(&target_directory))
}

fn get_canonical_path(path: &Path) -> FilePath {

    let file_path =  match path.canonicalize() {
        Ok(canonical_path) => String::from(canonical_path.to_str().unwrap_or_else(|| "")),
        Err(_) => String::from(""),
    };
    FilePath::from(&file_path)
}
