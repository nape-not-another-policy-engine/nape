use std::fs::File;
use std::io::Read;
use nape_kernel::error::{Error, Kind};
use nape_kernel::values::specification::v1_0_0::assurance_procedure::AssuranceProcedure;
use crate::gateway_adapter::serde::specification_serializer::assurance_procedure::v1_0_0::AssuranceProcedureFile;


/// # Overview
///
/// An implementation of the [`RetrieveAssuranceProcedure`] gateway which retrieves the [`AssuranceProcedure`] from an assurance procedure file serialized as a YAML on the machine's filesystem.
///
/// # Returns
///
/// A [`RetrieveAssuranceProcedure`] function which will attempt to retrieve an [`AssuranceProcedure`] from a file on the filesystem.
///
/// # Example
///
/// ```no_run
///
/// let result = from_yaml_on_filesystem("path/to/assurance_procedure.yaml");
///  // Now do something with the result
///
/// ```
pub fn from_yaml_on_filesystem(file_path: &str) -> Result<AssuranceProcedure, Error> {

        let mut file = File::open(file_path)
            .map_err(|e| Error::for_system(Kind::ProcessingFailure, format!("Could not open file: {}", e)))?;

        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .map_err(|e| Error::for_system(Kind::ProcessingFailure, format!("Could not read file: {}", e)))?;

        let assurance_procedure_file: AssuranceProcedureFile = serde_yaml::from_str(&file_content)
            .map_err(|e| Error::for_system(Kind::ProcessingFailure, format!("Could not deserialize file content: {}", e)))?;

        let assurance_procedure = assurance_procedure_file.try_to()
            .map_err(|e| Error::for_system(Kind::ProcessingFailure, format!("Failed to convert assurance procedure file content to AssuranceProcedure: {}", e)))?;

        Ok(assurance_procedure)

}
