use std::{env, fs};
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{PathBuf};

/// Checks if the NAPE Testing directory exists in the user's home directory, if it does not exist, it creates the directory '/nape_testing' in the user's home directory then returns the path to the directory.  If this directory already  exist, it returns the path to the directory.
///
/// The '/nape_testing' directory is used the place where any type of testing files or directories are created and stored during testing.  This directory is created in the user's home directory so that it is easy to find, remove, and manage because we assume the user has access to their home directory.
///
pub fn nape_testing_dir() -> PathBuf {
        let mut home_dir = env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .map(PathBuf::from)
            .expect("Could not get the user's home directory.");

        home_dir.push("nape_testing");

        if !home_dir.exists() {
            fs::create_dir_all(&home_dir)
                .expect("Could not create the NAPE Testing Root directory.");
        }

        home_dir

}

pub fn nape_testing_dir_as_string() -> String {
    nape_testing_dir()
        .canonicalize().expect("Could not canonicalize the path.")
        .to_str().expect("Could not convert the path to a string.").to_string()
}

/// Removes the NAPE Testing directory from the user's home directory.  If the directory does not exist, it does nothing.
///
pub fn remove_nape_testing_dir() {
    let nape_testing_dir = nape_testing_dir();

    if nape_testing_dir.exists() {
        fs::remove_dir_all(&nape_testing_dir)
            .expect(format!("Could not remove the NAPE Testing Root directory '{}'.", nape_testing_dir.display()).as_str());
    }
}

/// Allows you to pass a subpath into the function, and it will create either the directory, or the file, in the NAPE Testing directory with the subpath.  If the directory does not exist, it creates it, then returns the path to the directory.  If it does exist, it returns the path to the directory.
///
/// # Arguments
///
/// * `subpath` - The subpath to the directory or file you want to create in the NAPE Testing directory.
///
pub fn create_subpath(subpath: &str) -> PathBuf {
    let mut nape_test_root = nape_testing_dir();
    nape_test_root.push(subpath);

    if !nape_test_root.exists() {
        if nape_test_root.extension().is_some() {
            // It's a file
            if let Some(parent) = nape_test_root.parent() {
                fs::create_dir_all(parent)
                    .expect(format!("Could not create the parent directory for '{}'.", parent.display()).as_str());
                fs::File::create(&nape_test_root)
                    .expect(format!("Could not create the file '{}'.", nape_test_root.display()).as_str());
            }
        } else {
            // It's a directory
            fs::create_dir_all(&nape_test_root)
                .expect(format!("Could not create the directory '{}'.", nape_test_root.display()).as_str());
        }
    }

    nape_test_root
}

pub fn create_subpath_as_string(subpath: &str) -> String {
    create_subpath(subpath)
        .canonicalize().expect("Could not canonicalize the path.")
        .to_str().expect("Could not convert the path to a string.").to_string()
}

/// Allows you to pass a subpath into the function, and it will return the path to the directory or file in the NAPE Testing directory with the subpath.
///
/// This only returns the path to the directory or file, it does not create the directory or file.
pub fn generate_path_for(subpath: &str) -> PathBuf {
    let mut nape_test_root = nape_testing_dir();
    nape_test_root.push(subpath);
    nape_test_root
}

pub fn generate_nape_testing_file(subpath: &str, contents: &str) -> PathBuf {
    let mut nape_test_root = nape_testing_dir();
    nape_test_root.push(subpath);

    if !nape_test_root.exists() {
        if let Some(parent) = nape_test_root.parent() {
            fs::create_dir_all(parent)
                .expect(format!("Could not create the parent directory for '{}'.", parent.display()).as_str());
        }
        fs::write(&nape_test_root, contents)
            .expect(format!("Could not write to the file '{}'.", nape_test_root.display()).as_str());
    }

    nape_test_root
}

pub fn generate_nape_testing_file_write_only(subpath: &str, contents: &str) -> PathBuf {
    let mut nape_test_root = nape_testing_dir();
    nape_test_root.push(subpath);

    if !nape_test_root.exists() {
        if let Some(parent) = nape_test_root.parent() {
            fs::create_dir_all(parent)
                .expect(format!("Could not create the parent directory for '{}'.", parent.display()).as_str());
        }

        // Create the file with write-only permissions
        let mut file = OpenOptions::new()
            .write(true)
            .read(false)
            .create(true)
            .open(&nape_test_root)
            .expect(format!("Could not create the file '{}'.", nape_test_root.display()).as_str());

        file.write_all(contents.as_bytes())
            .expect(format!("Could not write to the file '{}'.", nape_test_root.display()).as_str());

        // Set the file permissions to write-only
        let mut perms = file.metadata().expect("Failed to get file metadata").permissions();
        perms.set_mode(0o200); // Write-only permissions
        fs::set_permissions(&nape_test_root, perms).expect("Failed to set file permissions");
    }

    nape_test_root
}

/// Removes the subpath from the NAPE Testing directory.  If the subpath does not exist, it does nothing.
///
/// # Arguments
///
/// * `subpath` - The subpath to the directory or file you want to remove from the NAPE Testing directory.
///
pub fn remove_subpath(subpath: &str) {
    let mut nape_test_root = nape_testing_dir();
    nape_test_root.push(subpath);

    if nape_test_root.exists() {
        if nape_test_root.is_dir() {
            fs::remove_dir_all(&nape_test_root)
                .expect(format!("Could not remove the directory '{}'.", nape_test_root.display()).as_str());
        } else {
            fs::remove_file(&nape_test_root)
                .expect(format!("Could not remove the file '{}'.", nape_test_root.display()).as_str());
        }
    }
}


/// Syntax sugar for the [`nape_testing_dir`] function, returns the path as a [`String`]
#[macro_export]
macro_rules! generate_nape_testing {
    () => {{
        nape_testing_filesystem::nape_testing_dir_as_string()
    }};
}

/// Syntax sugar for the `[remove_nape_testing_dir`] function.
#[macro_export]
macro_rules! remove_nape_testing {
    () => {{
        nape_testing_filesystem::remove_nape_testing_dir()
    }};
}

/// Syntax sugar for the [`generate_path_for`] function, returns the path as a [`PathBuf`]
///
/// # Arguments
///
/// * `subpath` - The subpath to the directory or file you want to create in the NAPE Testing directory.
///
#[macro_export]
macro_rules! path_for {
    ($subpath:expr) => {{
        nape_testing_filesystem::generate_path_for($subpath)
    }};
}

/// Syntax sugar for the [`create_subpath`] function, returns the path as a [`String`]
#[macro_export]
macro_rules! create {
    ($subpath:expr) => {{
        nape_testing_filesystem::create_subpath($subpath)
    }};
}

/// Syntax sugar for the [`generate_nape_testing_file`] function, returns the path as a [`PathBuf`]
///
/// Crates a file in the NAPE Testing directory with the given subpath and contents.
///
/// # Arguments
///
/// * `subpath` - The subpath to the file you want to create in the NAPE Testing directory.
/// * `contents` - The contents of the file as a [`String`].
///
/// # Returns
///
///  The path to the file as a [`PathBuf`].
///
/// # Example
///
/// ```no_run
/// use nape_testing_filesystem::create_file;
///
///  let local_file = create_file!("some_directory/file_to_create.yaml", "The text you want in the file.");
///```
///
#[macro_export]
macro_rules! create_file {
    ($subpath:expr, $contents:expr) => {{
        nape_testing_filesystem::generate_nape_testing_file($subpath, $contents)
    }};
    () => {};
}

/// Syntax sugar for the [`generate_nape_testing_file_write_only`] function, returns the path as a [`PathBuf`]
///
/// Creates a file in the NAPE Testing directory with the given subpath and contents that is write only. This is used for generating test files that **should not be read from**.
///
/// # Arguments
///
/// * `subpath` - The subpath to the file you want to create in the NAPE Testing directory.
/// * `contents` - The contents of the file as a [`String`].
///
/// # Returns
///
///  The path to the file as a [`PathBuf`].
///
/// # Example
///
/// ```no_run
///
/// use nape_testing_filesystem::create_write_only_file;;
///
///  let local_file = create_write_only_file!("some_directory/file_to_create.yaml", "The text you want in the file.");
///
///```
///
#[macro_export]
macro_rules! create_write_only_file {
    ($subpath:expr, $contents:expr) => {{
        nape_testing_filesystem::generate_nape_testing_file_write_only($subpath, $contents)
    }};
    () => {};
}

/// Syntax sugar for the [`remove_subpath`] function.
#[macro_export]
macro_rules! remove {
    ($subpath:expr) => {{
      nape_testing_filesystem::remove_subpath($subpath)
    }};
}

/// Compares the contents of a file, given its file path, to the expected contents.
///
/// The file must exist and be readable.  This macro will read the file contents and compare them to the expected contents.  If the file contents do not match the expected contents, the test will fail.
///
/// # Arguments
///
/// * `expected_contents` - The expected contents of the file as a [`String`].
/// * `path` - The path to the file as a [`PathBuff`].
///
#[macro_export]
macro_rules! file_contents_eq {
    ($expected_contents:expr, $path:expr) => {{
        use std::fs;
        use std::str;

        // Read the file contents
        let file_contents = fs::read_to_string($path)
            .expect("Could not read the file.");

        // Assert that the file contents match the expected contents
        assert_eq!(file_contents, $expected_contents, "{}", format!("The file contents do not match the expected contents.\n\tExpected:\t{}\n\tActual:\t{}", $expected_contents, file_contents));
    }};
}

/// Get the canonical path to a file from a [`PathBuf`].
///
/// # Arguments
///
/// * `path` - The path to the file as a [`PathBuf`].
///
/// # Returns
///
/// The canonical path as a [`String`].
#[macro_export]
macro_rules! canonical_path {
    ($path:expr) => {{
        $path.canonicalize().expect("Could not canonicalize the path.")
            .to_str().expect("Could not convert the path to a string.").to_string()
    }};
}