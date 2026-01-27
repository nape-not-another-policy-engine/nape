use kernel_oss::error::{Error, Kind};
use kernel_oss::values::directory::directory_list::DirectoryList;
use kernel_oss::values::specification::repository_link::RepositoryLink;
use std::fs;
use std::path::{Path, PathBuf};

pub fn retrieve_procedure_from_fs(
    repo_link: &RepositoryLink,
    procedure_directory: &str,
    download_directory: &str,
) -> Result<DirectoryList, Error> {
    let local_procedure_dir = parse_directory_url(&repo_link.to_string())
        .map_err(|error| {
            Error::for_system(
                Kind::ProcessingFailure,
                format!(
                    "Failed to parse repository link URL [{}]: {}",
                    repo_link.to_string(),
                    error.message
                ),
            )
        })?
        .join(procedure_directory);

    copy_dir_recursive(local_procedure_dir.as_path(), Path::new(download_directory)).map_err(
        |error| {
            Error::for_system(
                Kind::ProcessingFailure,
                format!(
                    "Failed to copy procedure directory from [{:?}] to [{:?}]: {}",
                    local_procedure_dir, download_directory, error.message
                ),
            )
        },
    )?;

    let directory_list = build_directory_list(download_directory).map_err(|error| {
        Error::for_system(
            Kind::ProcessingFailure,
            format!(
                "Failed to build directory list from downloaded procedure in [{}]: {}",
                download_directory, error.message
            ),
        )
    })?;

    Ok(directory_list)
}

fn parse_directory_url(uri: &str) -> Result<PathBuf, Error> {
    if uri.starts_with("file://") {
        let path_str = &uri[7..]; // Remove "file://" prefix
        Ok(PathBuf::from(path_str))
    } else {
        Err(Error::for_system(
            Kind::Unexpected,
            "Invalid URI scheme, expected file://",
        ))
    }
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), Error> {
    if !dst.exists() {
        fs::create_dir_all(dst).map_err(|error| {
            Error::for_system(
                Kind::ProcessingFailure,
                format!("Failed to create directory {:?}: {}", dst, error),
            )
        })?;
    }
    for entry in fs::read_dir(src).map_err(|error| {
        Error::for_system(
            Kind::ProcessingFailure,
            format!("Failed to read directory {:?}: {}", src, error),
        )
    })? {
        let entry = entry.map_err(|error| {
            Error::for_system(
                Kind::ProcessingFailure,
                format!("Failed to read directory entry in {:?}: {}", src, error),
            )
        })?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if entry
            .file_type()
            .map_err(|error| {
                Error::for_system(
                    Kind::ProcessingFailure,
                    format!("Failed to get file type for {:?}: {}", src_path, error),
                )
            })?
            .is_dir()
        {
            copy_dir_recursive(&src_path, &dst_path).map_err(|error| {
                Error::for_system(
                    Kind::ProcessingFailure,
                    format!(
                        "Failed to copy directory {:?} to {:?}: {}",
                        src_path, dst_path, error
                    ),
                )
            })?
        } else {
            fs::copy(&src_path, &dst_path).map_err(|error| {
                Error::for_system(
                    Kind::ProcessingFailure,
                    format!(
                        "Failed to copy file {:?} to {:?}: {}",
                        src_path, dst_path, error
                    ),
                )
            })?;
        }
    }

    Ok(())
}

fn build_directory_list(download_directory: &str) -> Result<DirectoryList, Error> {
    let process_def_doc_yaml_path = format!("{}/assurance_procedure.yaml", download_directory);
    if !Path::new(&process_def_doc_yaml_path).exists() {
        return Err(Error::for_system(Kind::GatewayError,
									 format!("The procedure definition document '{}' does not exist in the download directory '{}'.  Check that the procedure definition document exists in the repository.", process_def_doc_yaml_path, download_directory)));
    }

    let activity_directory_path = format!("{}/activity", download_directory);
    if !Path::new(&activity_directory_path).exists() {
        return Err(Error::for_system(Kind::GatewayError,
									 format!("The activity test directory '{}' does not exist in the download directory '{}'.  Check that the activity test directory exists in the repository.", activity_directory_path, download_directory)));
    }

    let mut directory_list = DirectoryList::default()
		.try_add("assurance-procedure-file", &process_def_doc_yaml_path)
		.map_err(|error| Error::for_system(Kind::GatewayError,
										   format!("Failed to add the procedure definition document '{}' to the directory list. {}", process_def_doc_yaml_path, error)))?;

    directory_list = directory_list
        .try_add("activity-dir", &activity_directory_path)
        .map_err(|error| {
            Error::for_system(
                Kind::GatewayError,
                format!(
                    "Failed to add the activity-test directory '{}' to the directory list. {}",
                    activity_directory_path, error
                ),
            )
        })?;

    Ok(directory_list)
}
