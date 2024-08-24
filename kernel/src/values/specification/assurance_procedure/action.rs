use crate::error::{Error, Kind};
use crate::values::specification::description::Description;
use crate::values::specification::file_path::FilePath;
use crate::values::specification::name::Name;
use crate::values::specification::short_description::ShortDescription;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Action {
    pub name: Name,
    pub short: ShortDescription,
    pub description: Description,
    pub test: FilePath,
    pub evidence: FilePath,
}

impl Action {
    pub fn builder() -> ActionBuilder {
        ActionBuilder::new()
    }
}

pub struct ActionBuilder {
    name: Option<String>,
    short: Option<String>,
    description: Option<String>,
    test: Option<String>,
    evidence: Option<String>,
}

impl ActionBuilder {
    pub fn new() -> ActionBuilder {
        ActionBuilder {
            name: None,
            short: None,
            description: None,
            test: None,
            evidence: None,
        }
    }

    pub fn name(mut self, name: &str) -> ActionBuilder {
        self.name = Some(name.to_string());
        self
    }

    pub fn short_description(mut self, short: &str) -> ActionBuilder {
        self.short = Some(short.to_string());
        self
    }

    pub fn long_description(mut self, description: &str) -> ActionBuilder {
        self.description = Some(description.to_string());
        self
    }

    pub fn test_file_path(mut self, test: &str) -> ActionBuilder {
        self.test = Some(test.to_string());
        self
    }

    pub fn evidence_file_path(mut self, evidence: &str) -> ActionBuilder {
        self.evidence = Some(evidence.to_string());
        self
    }

    pub fn try_build(self) -> Result<Action, Error> {

        let valid_name = self.build_name()?;
        let valid_short = self.build_short_description()?;
        let valid_description =self.build_long_description()?;
        let valid_test = self.build_test_file_path()?;
        let valid_evidence = self.build_evidence_file_path()?;

        Ok(Action  {
            name: valid_name,
            short: valid_short,
            description: valid_description,
            test: valid_test,
            evidence: valid_evidence
        })
    }

    fn build_name(&self) -> Result<Name, Error> {
        let name = self.name.as_ref()
            .ok_or(custom_error("The name is required, but was not provided."))?;
        Name::try_from(name)
            .map_err(|error| custom_error(&format!("There is an issue with the name '{}'. {}", name, error.message)))
    }

    fn build_short_description(&self) -> Result<ShortDescription, Error> {
        let short = self.short.as_ref()
            .ok_or(custom_error("The short description is required, but was not provided."))?;
        ShortDescription::try_from(short)
            .map_err(|error| custom_error(&format!("There is an issue with the short description '{}'. {}", short, error.message)))
    }

    fn build_long_description(&self) -> Result<Description, Error> {
        let description = self.description.as_ref()
            .ok_or(custom_error("The long description is required, but was not provided."))?;
        Description::try_from(description)
            .map_err(|error| custom_error(&format!("There is an issue with the long description '{}'. {}", description, error.message)))
    }

    fn build_test_file_path(&self) -> Result<FilePath, Error> {
        let test = self.test.as_ref()
            .ok_or(custom_error("The test file path is required, but was not provided."))?;
        FilePath::try_from(test)
            .map_err(|error| custom_error(&format!("There is an issue with the test file path '{}'. {}", test, error.message)))
    }

   fn build_evidence_file_path(&self) -> Result<FilePath, Error> {
        let evidence = self.evidence.as_ref()
            .ok_or(custom_error("The evidence file path is required, but was not provided."))?;
        FilePath::try_from(evidence)
            .map_err(|error| custom_error(&format!("There is an issue with the evidence file path '{}'. {}", evidence, error.message)))
    }

}

fn custom_error(message: &str) -> Error {
    Error::for_user(Kind::InvalidInput,
                    format!("The Action for an Assurance Procedure could not be created. {}", message))
}



