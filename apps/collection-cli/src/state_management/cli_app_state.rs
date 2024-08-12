use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use nape_kernel::error::{Error, Kind};
use nape_kernel::values::directory::directory_list::DirectoryList;
use nape_kernel::values::specification::metadata::MetaData;
use nape_kernel::values::specification::procedure::Procedure;
use nape_kernel::values::specification::subject::Subject;

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct CLIAppState {
    pub subject_nrn: String,
    pub subject_id: String,
    pub procedure_repository: String,
    pub procedure_directory: String ,
    pub metadata: HashMap<String, String>,
    pub directories: HashMap<String, String>,
}

impl CLIAppState {

    pub fn builder() -> CLIAppStateBuilder {
        CLIAppStateBuilder::default()
    }

    pub fn try_directory_list(self) -> Result<DirectoryList, Error> {
        DirectoryList::try_from_hashmap(self.directories)
            .map_err(|error| Error::for_system(Kind::GatewayError,
                                               format!("Failed to convert the CLIAppState directories to a DirectoryList. {}", error.message)))
    }
}


#[derive(Clone, Default)]
pub struct CLIAppStateBuilder {
    subject: Option<Subject>,
    procedure: Option<Procedure>,
    metadata: Option<MetaData>,
    directory_list: Option<DirectoryList>
}

impl CLIAppStateBuilder {

    pub fn for_subject(mut self, subject: &Subject) -> Self {
        self.subject = Some(subject.clone());
        self
    }
    pub fn with_procedure(mut self, procedure: &Procedure) -> Self {
        self.procedure = Some(procedure.clone());
        self
    }
    pub fn with_metadata (mut self, meta_data: &MetaData) -> Self {
        self.metadata = Some(meta_data.clone());
        self
    }
    pub fn with_directory_list(mut self, directory_list: &DirectoryList) -> Self {
        self.directory_list = Some(directory_list.clone());
        self
    }

    pub fn try_build(&self) -> Result<CLIAppState, Error> {

        let directory_list = self.validate_directory_list()?;
        let subject = self.validate_subject()?;
        let procedure  = self.validate_procedure()?;
        let metadata = self.validate_metadata()?;
        Ok(
            CLIAppState {
                metadata,
                subject_nrn: subject.nrn.value,
                subject_id: subject.id.value,
                procedure_repository: procedure.repository.to_string(),
                procedure_directory: procedure.directory.to_string(),
                directories: directory_list,
            }
        )

    }

    fn validate_subject(&self) -> Result<Subject, Error> {
        Ok (
            self.subject.as_ref()
            .ok_or(self.custom_error("A Subject is required, although one was not provided."))?
            .clone()
        )
    }
    fn validate_procedure(&self) -> Result<Procedure, Error> {
        Ok (
            self.procedure.as_ref()
            .ok_or(self.custom_error("A Procedure is required, although one was not provided."))?
            .clone()
        )
    }
    fn validate_metadata(&self) -> Result<HashMap<String, String>, Error> {
        Ok (
            self.metadata.as_ref()
            .ok_or(self.custom_error("Metadata is required, although none was provided."))?
            .data.iter().map(|(name, desc)| ( name.value.clone(), desc.value.clone() )  )
            .collect()
        )
    }
    fn validate_directory_list(&self) -> Result<HashMap<String, String>, Error> {
        Ok(
            self.directory_list.as_ref()
            .ok_or(self.custom_error("A Directory List is required, although one was note provided."))?
            .paths.iter().map( |(path_name, path_value)| ( path_name.clone(), path_value.clone() ) )
            .collect()
        )
    }

    fn custom_error(&self, message: &str) -> Error {
        Error::for_system(Kind::InvalidInput,
                        format!("There is an issue establishing the CLI Application State. {}", message))
    }

}
