use nape_kernel::error::{Error, Kind};
use nape_kernel::values::specification::procedure::Procedure;
use nape_kernel::values::specification::subject::Subject;
use nape_kernel::values::specification::metadata::MetaData;

pub mod request {
    use super::*;

    ///  # Overview
/// Contains all the necessary data for the  [`EvaluateEvidenceCollection`] usecase to process..
///
/// # Attributes
///
/// * `subject` - A reference to the [`Subject`] of the evidence collection.
/// * `procedure` - A reference to the [`Procedure`] of the evidence collection.
/// * `metadata` - A reference to the [`MetaData`] of the evidence collection.
///
/// # Design Decision
///
///  - All attributes are private so no one can instantntiate the struct directly without using the builder.
///  - The builder pattern is used to apply all validation logic and create an instance of the struct.
///
    #[derive(Clone, Debug)]
    pub struct EvaluateEvidence {
        subject: Subject,
        procedure: Procedure,
         metadata: MetaData,
    }

    impl EvaluateEvidence {

        /// Returns a new instance of the [`EvaluateEvidenceBuilder`] struct.
        ///
        /// Use this to create a new instance of the [`EvaluateEvidence`] struct.
        pub fn builder() -> EvaluateEvidenceBuilder<'static> {
            EvaluateEvidenceBuilder {
                subject_nrn: None,
                subject_id: None,
                procedure_dir: None,
                procedure_repo: None,
                metadata: Vec::new(),
            }
        }

        /// Returns a reference to the [`Subject`] of the [`EvaluateEvidence`] struct.
        pub fn subject(&self) -> &Subject {
            &self.subject
        }

        /// Returns a reference to the [`Procedure`] of the [`EvaluateEvidence`] struct.
        pub fn procedure(&self) -> &Procedure {
            &self.procedure
        }

        /// Returns a reference to the [`MetaData`] of the [`EvaluateEvidence`] struct.
        pub fn metadata(&self) -> &MetaData {
            &self.metadata
        }

    }

    pub struct EvaluateEvidenceBuilder<'a> {
        subject_nrn: Option<&'a str>,
        subject_id: Option<&'a str>,
        procedure_dir: Option<&'a str>,
        procedure_repo: Option<&'a str>,
        metadata: Vec<(String, String)>,
    }
    impl<'a> EvaluateEvidenceBuilder<'a> {

        pub fn subject_nrn(&mut self, nrn: &'a str) -> &mut Self {
            self.subject_nrn = Some(nrn);
            self
        }

        pub fn subject_id(&mut self, id: &'a str) -> &mut Self {
            self.subject_id = Some(id);
            self
        }

        pub fn procedure_directory(&mut self, directory_path: &'a str) -> &mut Self {
            self.procedure_dir = Some(directory_path);
            self
        }

        pub fn procedure_repository(&mut self, repo_link: &'a str) -> &mut Self {
            self.procedure_repo = Some(repo_link);
            self
        }

        pub fn metadata(&mut self, metadata: &Vec<(String, String)>) -> &mut Self {
            self.metadata = metadata.clone();
            self
        }

        pub fn try_build(&self) -> Result<EvaluateEvidence, Error> {

            let subject = validate_subject(self.subject_nrn, self.subject_id).map_err( custom_error)?;
            let procedure = validate_procedure(self.procedure_repo, self.procedure_dir).map_err(custom_error)?;
            let metadata = validate_metadata(&self.metadata).map_err(custom_error)?;

           Ok( EvaluateEvidence { subject, procedure, metadata } )

        }

    }

    fn validate_subject(nrn: Option<&str>, id: Option<&str>) -> Result<Subject, Error> {
        let nrn = nrn.ok_or_else(|| Error::for_user(Kind::InvalidInput,
                                                    "The NAPE Resource Name (NRN) of the Subject was not provided.".to_string()))?;
        let id = id.ok_or_else(|| Error::for_user(Kind::InvalidInput,
                                                  "The Subject Id was not provided.".to_string()))?;
        Subject::try_new(nrn, id).map_err(|e| Error::for_user(Kind::InvalidInput,
        format!("There is an issue with the Subject data you provided. {}", e.message)))
    }

    fn validate_procedure(repo_link: Option<&str>, directory: Option<&str>) -> Result<Procedure, Error> {
        let repo_link = repo_link.ok_or_else(|| Error::for_user(Kind::InvalidInput,
                                                                "The repository link of the Procedure was not provided.".to_string()))?;
        let directory = directory.ok_or_else(|| Error::for_user(Kind::InvalidInput,
                                                                "The directory path to the Procedure was not provided.".to_string()))?;
        Procedure::try_new(repo_link, directory).map_err(|e| Error::for_user(Kind::InvalidInput,
        format!("There is an issue with the Procedure data you provided. {}", e.message)))
    }

    fn validate_metadata(metadata: &Vec<(String, String)>) -> Result<MetaData, Error> {
        let mut valid_metadata = MetaData::default();
        for (key, value) in metadata {
            valid_metadata.add(key, value).map_err(|e| Error::for_user(Kind::InvalidInput,
            format!("There is an issue with the Metadata you provided. {}", e.message)))?;
        }
        Ok(valid_metadata)
    }

    fn custom_error(error: Error) -> Error {
        Error::for_user(error.kind,
                        format!("We are unable to create a valid Evaluate Evidence request. {}", error.message))

    }

}
