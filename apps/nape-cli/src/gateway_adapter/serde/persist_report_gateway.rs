use std::fs;
use std::fs::File;
use std::io::{Write};
use std::path::Path;
use nape_kernel::error::{Error, Kind};
use nape_kernel::values::specification::file_path::FilePath;
use nape_kernel::values::specification::traits::AssuranceReport;
use crate::gateway_adapter::serde::specification_serializer::assurance_report;


const FILE_NAME: &str = "assurance_report.yaml";

/// Implementation of the [`PersistReportGateway`] trait that saves an assurance report as a YAML file.
pub fn save_report_as_yaml(report: &dyn AssuranceReport, report_directory: &str) -> Result<FilePath, Error> {
        let directory_path = Path::new(report_directory);
        verify_dir_exists(directory_path)?;
        verify_dir_is_directory(directory_path)?;
        verify_dir_is_writable(directory_path)?;

        let file_path = format!("{}/{}", report_directory, FILE_NAME);

        let yaml = assurance_report::factory::create(report)?;

        let mut file = File::create(&file_path)
            .map_err(|e| Error::for_system(Kind::ProcessingFailure, format!("Could not create file: {}", e)))?;

        file.write_all(yaml.as_bytes())
            .map_err(|e| Error::for_system(Kind::ProcessingFailure, format!("Could not write to file: {}", e)))?;

        Ok(FilePath::from(&file_path))
}

fn verify_dir_exists(directory_path: &Path) -> Result<(), Error> {
    if !directory_path.exists() {
        return Err(Error::for_system(Kind::InvalidInput, format!("The directory '{}' does not exist.  Please ensure it has been created first.  Once thie directory has been created, you then create the assurance report in this directory.", directory_path.display())));
    }
    Ok(())
}

fn verify_dir_is_directory(directory_path: &Path) -> Result<(), Error> {
    if !directory_path.is_dir() {
        return Err(Error::for_system(Kind::InvalidInput, format!("The directory '{}' not a directory.  Please provide a a path to a valid directory where the assurance report will be created..", directory_path.display())));
    }
    Ok(())
}

fn verify_dir_is_writable(directory_path: &Path) -> Result<(), Error> {
    let dir_metadata = fs::metadata(directory_path)
        .map_err(|e| Error::for_system(Kind::NotFound,
                                       format!("Could not get information for '{}' .  Please ensure you the directory exists, and you have access to the directory. '{}", directory_path.display(), e)))?;

    let permissions = dir_metadata.permissions();
    if permissions.readonly() {
        return Err(Error::for_system(Kind::PermissionDenied,
                                     format!("The directory '{}' is not writable.  Please ensure you have write access to this directory.", directory_path.display())));
    }
    Ok(())
}
