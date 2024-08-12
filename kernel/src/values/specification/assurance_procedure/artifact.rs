use crate::error::{Error, Kind};
use crate::values::specification::description::Description;
use crate::values::specification::metadata::MetaData;
use crate::values::specification::name::Name;


/// The [`Artifact`] describes an expected artifact that is associated with a ['AssurnaceProcessDefinition'].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Artifact {
    pub name: Name,
    pub description: Description,
    pub expected_metadata: MetaData,
}

impl Artifact {

    /// Create a new [`Artifact`] with the given name, description, and expected metadata.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the artifact. The ['Name'] constraints are used to validate the name.
    /// * `description` - The description of the artifact. The ['Description'] constraints are used to validate the description.
    /// * `expected_metadata` - The expected metadata of the artifact. The ['MetaData'] constraints are used to validate the metadata.
    ///
    /// # Returns
    ///
    /// Either an [`Error`] or a new [`Artifact`] instance.
    ///
    /// # Errors
    ///
    /// The following reasons can cause an error:
    /// * If the name is invalid per the ['Name'] constraints.
    /// * If the description is invalid per the ['Description'] constraints.
    /// * If the metadata is invalid per the ['MetaData'] constraints.
    ///
    pub fn new(name: &str, description: &str, expected_metadata: &Vec<(String, String)> ) -> Result<Artifact, Error> {

        let validated_name = validate_name(name)?;
        let validated_description = validate_description(description)?;
        let validated_metadata = validate_metadata(&expected_metadata)?;

        Ok(Artifact {
            name: validated_name,
            description: validated_description,
            expected_metadata: validated_metadata,
        })

    }

}

fn validate_name(name: &str) -> Result<Name, Error> {
    Name::try_from(name).map_err(|error|
        Error::for_user(Kind::InvalidInput, format!("There is an issue with your artifact name '{}': {}", name, error)))
}

fn validate_description(desc: &str) -> Result<Description, Error> {
    Description::try_from(desc).map_err(|error|
        Error::for_user(Kind::InvalidInput, format!("There is an issue with your artifact description '{}': {}", desc, error)))
}

fn validate_metadata(metadata: &Vec<(String, String)>) -> Result<MetaData, Error> {
    let mut validated_metadata = MetaData::default();
    for (key, value) in metadata {
        validated_metadata.add(key.as_str(), value.as_str())
            .map_err(|error|
                Error::for_user(Kind::InvalidInput, format!("There is an issue with your artifact metadata '{}': {}", key, error)))?;
    }
    Ok(validated_metadata)
}
