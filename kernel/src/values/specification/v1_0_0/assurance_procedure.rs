use crate::error;
use crate::error::{Error};
use crate::values::specification::api_version::APIVersion;
use crate::values::specification::assurance_procedure::artifact::Artifact;
use crate::values::specification::assurance_procedure::artifacts::Artifacts;
use crate::values::specification::assurance_procedure::activity::Activity;
use crate::values::specification::assurance_procedure::activities::Activities;
use crate::values::specification::assurance_procedure::procedure::Procedure;
use crate::values::specification::kind::Kind;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AssuranceProcedure {
    pub api_version: APIVersion,
    pub kind: Kind,
    pub procedure: Procedure,
    pub activities: Activities,
    pub artifacts: Artifacts,
}

impl AssuranceProcedure {
    pub fn builder() -> AssuranceProcedureBuilder {
        AssuranceProcedureBuilder::new()
    }

}

 pub struct AssuranceProcedureBuilder {
    api_version: Option<String>,
    metadata: Vec<(String, String)>,
    procedure_nrn: Option<String>,
    procedure_short_desc: Option<String>,
    procedure_long_desc: Option<String>,
    activities: Vec<Activity>,
    artifacts: Vec<Artifact>
}

impl AssuranceProcedureBuilder {

    pub fn new() -> Self {
            AssuranceProcedureBuilder {
                api_version: None,
                metadata: Vec::new(),
                procedure_nrn: None,
                procedure_short_desc: None,
                procedure_long_desc: None,
                activities: Vec::new(),
                artifacts: Vec::new()
            }
    }

    pub fn api_version(mut self, api_version: &str) -> Self {
        self.api_version = Some(api_version.to_string());
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

    pub fn procedure_info(mut self, nrn: &str, short: &str, long: &str) -> Self {
        self.procedure_nrn = Some(nrn.to_string());
        self.procedure_short_desc = Some(short.to_string());
        self.procedure_long_desc = Some(long.to_string());
        self
    }

    pub fn add_activity(mut self, activity: &Activity) -> Self {
        self.activities.push(activity.clone());
        self
    }

    pub fn add_artifact(mut self, artifact: &Artifact) -> Self {
        self.artifacts.push(artifact.clone());
        self
    }

    pub fn try_build(&self) -> Result<AssuranceProcedure, Error> {
        let api_version = self.build_api_version()?;
        let kind = Kind::AssuranceProcedure;
        let procedure = self.build_procedure()?;
        let activities = self.build_activities();
        let artifacts = self.build_artifacts()?;

        Ok(AssuranceProcedure { api_version, kind, procedure, activities, artifacts })

    }

    fn build_api_version(&self) -> Result<APIVersion, Error> {
        let api_version = self.api_version.as_ref()
            .ok_or( custom_error("The APIVersion is required, but was not provided."))?;
        APIVersion::from_str(api_version)
            .map_err(|e| custom_error( format!("The APIVersion has an issue: '{}'", e.message.as_str()).as_str()) )
    }

    fn build_procedure(&self) -> Result<Procedure, Error> {
        let error_message = "Check that you provided the procedure info. Either the procedure NRN, short description, or long description is missing.";


        let nrn = self.procedure_nrn.as_ref()
            .ok_or( custom_error(error_message))?;
        let short = self.procedure_short_desc.as_ref()
            .ok_or( custom_error(error_message))?;
        let long = self.procedure_long_desc.as_ref()
            .ok_or( custom_error(error_message))?;

        Procedure::new(nrn.as_str(), short.as_str(), long.as_str())
            .map_err(|e| custom_error( format!("The procedure information has an issue: '{}'", e.message.as_str()).as_str()) )

    }

    fn build_activities(&self) -> Activities {
        let mut activities = Activities::default();
        for procedure in &self.activities {
           activities = activities.merge(procedure)
        }
        activities
    }

    fn build_artifacts(&self) -> Result<Artifacts, Error> {
        let mut artifacts = Artifacts::default();
        for artifact in &self.artifacts {
            artifacts = artifacts.merge(artifact).map_err(|e|
                custom_error(format!("The artifact '{}' has an issue: {} ", artifact.name.value.as_str(), e.message.as_str()).as_str()) )?;
        }
        Ok(artifacts)
    }

}

fn custom_error(message: &str) -> Error {
    Error::for_user(error::Kind::InvalidInput,
                    format!("The AssuranceProcedure could not be created: {}", message))
}
