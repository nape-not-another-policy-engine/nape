
pub mod request {
    use nape_kernel::error;
    use nape_kernel::error::Error;
    use nape_kernel::values::specification::api_version::APIVersion;
    use nape_kernel::values::specification::kind::Kind;
    use nape_kernel::values::specification::metadata::MetaData;
    use nape_kernel::values::specification::procedure::Procedure;
    use nape_kernel::values::specification::subject::Subject;
    use nape_kernel::values::time::start_time::StartTime;

    /// The [`StartProcedure`] defines the beginning of an evidence collection procedure for a business procedure.
    #[derive(Clone, Debug)]
    pub struct StartProcedure {
        pub start_time: StartTime,
        pub api_version: APIVersion,
        pub kind: Kind,
        pub metadata: MetaData,
        pub subject: Subject,
        pub procedure: Procedure,
    }

    #[derive(Clone, Debug, Default)]
    pub struct StartProcedureBuilder {
        api_version: Option<String>,
        subject_nrn: Option<String>,
        subject_id: Option<String>,
        procedure_repository: Option<String>,
        procedure_directory: Option<String>,
        metadata: Vec<(String, String)>,
        start_time: Option<u128>,
        start_now: bool
    }

    impl StartProcedureBuilder {
        pub fn api_version(mut self, api_version: &str) -> Self {
            self.api_version = Some(api_version.to_string());
            self
        }

        pub fn subject_nrn(mut self, nrn: &str ) ->  Self {
            self.subject_nrn = Some(nrn.to_string());
            self
        }

        pub fn subject_id(mut self, subject_id: &str) ->  Self {
            self.subject_id = Some(subject_id.to_string());
            self
        }

        pub fn procedure_repository(mut self, repo_link: &str) ->  Self {
            self.procedure_repository = Some(repo_link.to_string());
            self
        }

        pub fn procedure_directory(mut self, directory_path: &str) ->  Self {
            self.procedure_directory = Some(directory_path.to_string());
            self
        }

        pub fn add_metadata(mut self, key: &str, value: &str) ->Self {
            self.metadata.push((key.to_string(), value.to_string() ) );
            self
        }

        pub fn merge_metadata(mut self, metadata: &Vec<(String,String)>) -> Self {
            self.metadata.extend(metadata.clone());
            self
        }

        pub fn start_now(mut self) -> Self {
            self.start_now = true;
            self
        }

        pub fn start_at(mut self, start_time: u128) -> Self {
            self.start_time = Some(start_time);
            self
        }

        pub fn try_build(&self) -> Result<StartProcedure, Error> {

            let start_time = self.validate_start_time()?;
            let api_version = self.validate_api_version()?;
            let kind = Kind::AssuranceProcedure;
            let subject = self.validate_subject()?;
            let procedure = self.validate_procedure()?;
            let metadata = self.validate_metadata(&start_time)?;

            Ok(StartProcedure { start_time, api_version, kind, metadata, subject, procedure })
        }

        fn validate_start_time(&self) -> Result<StartTime, Error> {
            if self.start_now { return Ok(StartTime::now()) }
            let start_time = self.start_time.as_ref().ok_or(
                self.custom_error("The Start Time is required, although it was not provided."))?;
            StartTime::try_from(start_time.clone()).map_err(|e|
                self.custom_error(&format!("The Start Time has an issue. {}", e.message)))
        }
        fn validate_api_version(&self) -> Result<APIVersion, Error> {
            let api_version = self.api_version.as_ref().
                ok_or(self.custom_error("The API Version is required, but was not provided."))?;

            APIVersion::from_str(api_version).map_err(|e|
                self.custom_error(&format!("The APIVersion has an issue. '{}'", e.message)))
        }

        fn validate_subject(&self) -> Result<Subject, Error> {
            let nrn = self.subject_nrn.as_ref()
                .ok_or(self.custom_error("The Subject NRN is required, although it was not provided."))?;
            let subject_id = self.subject_id.as_ref()
                .ok_or(self.custom_error("The Subject ID is required, although it was not provided."))?;
            Subject::try_new(nrn, subject_id).map_err(|e|
                self.custom_error(&format!("There is an issue with your Subject data. '{}'", e.message.as_str())))
        }

        fn validate_procedure(&self) -> Result<Procedure, Error> {
            let repo_link = self.procedure_repository.as_ref()
                .ok_or(self.custom_error("The procedure repository link is required, although it was not provided."))?;
            let directory = self.procedure_directory.as_ref()
                .ok_or(self.custom_error("The procedure directory is required, although it was not provided."))?;
            Procedure::try_new(repo_link, directory).map_err(|e|
                self.custom_error(&format!("There is an issue with your Procedure data. {}", e.message.as_str())))
        }

        fn validate_metadata(&self, start_time: &StartTime) -> Result<MetaData, Error> {
            let mut metadata = MetaData::default();
            for (key, value) in self.metadata.iter() {
                metadata.add(key, value).map_err(|e|
                    self.custom_error(&format!("The Metadata has an issue. {}", e.message.as_str())))?;
            }
            metadata.add("utc-start", &start_time.to_string())?;
            Ok(metadata)
        }

        fn custom_error(&self, message: &str) -> Error {
            Error::for_user(error::Kind::InvalidInput,
                            format!("There is an issue with your Start Procedure request. {}", message))
        }

    }

}


pub mod response {
    use nape_kernel::values::directory::directory_list::DirectoryList;
    use nape_kernel::values::specification::api_version::APIVersion;
    use nape_kernel::values::specification::kind::Kind;
    use nape_kernel::values::specification::metadata::MetaData;
    use nape_kernel::values::specification::procedure::Procedure;
    use nape_kernel::values::specification::subject::Subject;

    #[derive(Clone, Debug)]
    pub struct ProcedureStarted {
        pub api_version: APIVersion,
        pub kind: Kind,
        pub subject: Subject,
        pub procedure: Procedure,
        pub metadata: MetaData,
        pub directory_list: DirectoryList
    }
}



