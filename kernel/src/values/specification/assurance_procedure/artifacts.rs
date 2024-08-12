use crate::error::{Error, Kind};
use crate::values::specification::assurance_procedure::artifact::Artifact;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Artifacts {
    artifacts: Vec<Artifact>
}

impl Artifacts {

    pub fn add(self, name: &str, description: &str, expected_metadata: &Vec<(String, String)>) -> Result<Artifacts, Error> {
        if self.artifacts.iter().any(|existing_artifact| existing_artifact.name.value == name) {
            return Err(
                Error::for_user(Kind::InvalidInput,
                                format!("The artifact  '{}' cannot be added because it already exists.", name)));
        }
        let valid_artifact = validate_artifact(name, description, expected_metadata)?;
        let mut new_artifacts = self.clone();
        new_artifacts.artifacts.push(valid_artifact);
        Ok(new_artifacts)
    }

    pub fn merge(self, artifact: &Artifact) -> Result<Artifacts, Error> {
        if self.artifacts.iter().any(|existing_artifact| existing_artifact.name == artifact.name) {
            return Err(
                Error::for_user(Kind::InvalidInput,
                                format!("The artifact '{}' cannot be added because an artifact with the name '{}' already exists.", artifact.name.value, artifact.name.value)));
        }
        let mut new_artifacts = self.clone();
        new_artifacts.artifacts.push(artifact.clone());
        Ok(new_artifacts)
    }

    pub fn count(self) -> usize {
        self.artifacts.len()
    }

}

fn validate_artifact(name: &str, description: &str, expected_metadata: &Vec<(String, String)>) -> Result<Artifact, Error> {
    Artifact::new(name, description, expected_metadata).map_err(|error|
        Error::for_user(Kind::InvalidInput,
                        format!("There is an issue adding your artifact: {}", error)))
}
