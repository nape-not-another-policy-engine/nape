use crate::algorithms::signature_algorithm::Signature;
use crate::error::{Error, Kind};
use crate::values::specification::assurnace_report::signed_file::SignedFile;
use crate::values::specification::description::Description;
use crate::values::specification::name::Name;
use crate::values::specification::outcome::Outcome;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Action {
    name: Name,
    outcome: Outcome,
    reason: Description,
    test_file: SignedFile,
    evidence_file: SignedFile
}

impl Action {

    /// Create a new instance of the [`ActionBuilder`].
    ///
    /// This is the only way to instantiate a new [`Action`] instance.
    pub fn builder() -> ActionBuilder {
        ActionBuilder::new()
    }

    /// Return a reference to the [`Name`] of the action.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Return a reference to the [`Outcome`] of the action.
    pub fn outcome(&self) -> &Outcome {
        &self.outcome
    }

    /// Return a reference to the reason ( a [`Description`] struct) of the action.
    pub fn reason(&self) -> &Description {
        &self.reason
    }

    /// Return a reference to the test file (a [`SignedFile`] struct) of the action.
    pub fn test_file(&self) -> &SignedFile {
        &self.test_file
    }

    /// Return a reference to the evidence file (a [`SignedFile`] struct) of the action.
    pub fn evidence_file(&self) -> &SignedFile {
        &self.evidence_file
    }

}


/// The [`ActionBuilder`] struct is used to create a new instance of the [`Action`] struct.  This builder applies all the validation logic to the data that is used to create the [`Action`] instance.
///
pub struct ActionBuilder {
    name: Option<Name>, // note - if this is not provided, then the name must be provided
    name_str: Option<String>,
    outcome: Option<Outcome>, // note - if this is not provided, then the outcome must be provided
    outcome_str: Option<String>,
    reason: Option<Description>, // note - if this is not provided, then the reason must be provided
    reason_str: Option<String>,
    signed_test: Option<SignedFile>, // Note - if this is not provided, then the test_file_path and test_file_signature must be provided
    test_file_path_str: Option<String>,
    test_file_signature_str: Option<String>,
    signed_evidence: Option<SignedFile>, // note - if this is not provided, then the evidence_file_path and evidence_file_signature must be provided
    evidence_file_path_str: Option<String>,
    evidence_file_signature_str: Option<String>
}

///
/// The [`ActionBuilder`] struct is used to create a new instance of the [`Action`] struct.  This builder applies all the validation logic to the data that is used to create the [`Action`] instance.
///
/// # Design Decision
///
/// Any method that stars with *use_* will take precedent over its counterpart because the *use_* methods accept valid instances of the data that is being used to build the [`Action`] instance.
///
impl ActionBuilder {
    /// Create a new instance of the [`ActionBuilder`].
    pub fn new() -> Self {
        Self {
            name: None,
            name_str: None,
            outcome: None,
            outcome_str: None,
            reason: None,
            reason_str: None,
            signed_test: None,
            test_file_path_str: None,
            test_file_signature_str: None,
            signed_evidence: None,
            evidence_file_path_str: None,
            evidence_file_signature_str: None
        }
    }

    /// Use an existing ['Name'] instance to set the name of the action.
    pub fn use_name(mut self, name: &Name) -> Self {
        self.name = Some(name.clone());
        self
    }

    /// Use a string to set the name of the action.
    pub fn name(mut self, name: &str) -> Self {
        self.name_str = Some(name.to_string());
        self
    }

    /// Use an existing['Outcome'] instance to set the outcome of the action.
    pub fn use_outcome(mut self, outcome: &Outcome) -> Self {
        self.outcome = Some(outcome.clone());
        self
    }

    /// Use a string to set the outcome of the action.
    pub fn outcome(mut self, outcome: &str) -> Self {
        self.outcome_str = Some(outcome.to_string());
        self
    }

    /// Use an existing ['Description'] instance to set the reason of the action.
    pub fn use_reason(mut self, reason: &Description) -> Self {
        self.reason = Some(reason.clone());
        self
    }

    /// Use a string to set the reason of the action.
    pub fn reason(mut self, reason: &str) -> Self {
        self.reason_str = Some(reason.to_string());
        self
    }

    /// Use an existing ['SignedFile'] instance to set the test file of the action.
    ///
    ///  If data is provided via this method, this data will take priority over any data provided via the test_file_path and test_file_signature methods when building the [`Action`] instance.
    pub fn use_test_file_signature(mut self, test_file: &SignedFile) -> Self {
        self.signed_test = Some(test_file.clone());
        self
    }

    /// Use a string to set the test file path.
    ///
    /// This method is commonly used when consuming data directly from a file or datastore, and is used in conjunction with the test_file_signature method.
    pub fn test_file_path(mut self, path: &str) -> Self {
        self.test_file_path_str = Some(path.to_string());
        self
    }

    /// Provide the signature for the test file as a string
    ///
    ///  This method is commonly used when consuming data directly from a file or datastore., and is used in conjunction with the test_file_path method.
    pub fn test_file_signature(mut self, signature: &str) -> Self {
        self.test_file_signature_str = Some(signature.to_string());
        self
    }

    /// Use an existing ['SignedFile'] instance to set the evidence file of the action.
    ///
    /// This method is used when building an [`Action`] instance from an existing [`SignedFile`] instance.
    ///
    ///  If data is provided via this method, this data will take priority over any data provided via the evidence_file_path and evidence_file_signature methods when building the [`Action`] instance.
    pub fn use_evidence_file_signature(mut self, evidence_file: &SignedFile) -> Self {
        self.signed_evidence = Some(evidence_file.clone());
        self
    }

    /// Use a string to set the evidence file path of the action.
    ///
    /// This method is commonly used when consuming data directly from a file or datastore, and is used in conjunction with the evidence_file_signature method.
    pub fn evidence_file_path(mut self, file_path: &str) -> Self {
        self.evidence_file_path_str = Some(file_path.to_string());
        self
    }

    /// Provide the signature for the evidence file as a string
    ///
    /// This method is commonly used when consuming data directly from a file or datastore, and is used in conjunction with the evidence_file_path method.
    pub fn evidence_file_signature(mut self, signature: &str) -> Self {
        self.evidence_file_signature_str = Some(signature.to_string());
        self
    }


    pub fn try_build(self) -> Result<Action, Error> {
        let valid_name = self.validate_name()?;
        let valid_outcome = self.validate_outcome()?;
        let valid_reason = self.validate_reason()?;
        let valid_test = self.validate_signed_test_file()?;
        let valid_evidence = self.validate_signed_evidence_file()?;

        Ok(Action {
            name: valid_name,
            outcome: valid_outcome,
            reason: valid_reason,
            test_file: valid_test,
            evidence_file: valid_evidence
        })
    }

     fn validate_name(&self) -> Result<Name, Error> {
        match &self.name {
            Some(name) => Ok(name.clone()),
            None => match &self.name_str {
                Some(name_str) => {
                    Name::try_from(name_str).map_err(|e| Error::for_user(Kind::InvalidInput,
                                                                         format!("There is an issue with the name '{}'. {}", name_str, e.message)))
                }
                None => Err(Error::for_user(Kind::InvalidInput, "Please provide a name for the action.".to_string()))
            }
        }
    }

     fn validate_outcome(&self) -> Result<Outcome, Error> {
        match &self.outcome {
            Some(outcome) => Ok(outcome.clone()),
            None => match &self.outcome_str {
                Some(outcome_str) => {
                    Outcome::try_from(outcome_str).map_err(|e| Error::for_user(Kind::InvalidInput,
                                                                               format!("There is an issue with the outcome '{}'. {}", outcome_str, e.message)))
                }
                None => Err(Error::for_user(Kind::InvalidInput, "Please provide an outcome for the action.".to_string()))
            }
        }
    }

     fn validate_reason(&self) -> Result<Description, Error> {
        match &self.reason {
            Some(reason) => Ok(reason.clone()),
            None => match &self.reason_str {
                Some(reason_str) => {
                    Description::try_from(reason_str).map_err(|e| Error::for_user(Kind::InvalidInput,
                                                                                  format!("There is an issue with the reason '{}'. {}", reason_str, e.message)))
                }
                None => Err(Error::for_user(Kind::InvalidInput, "Please provide a reason for the action.".to_string()))
            }
        }
    }

     fn validate_signed_test_file(&self) -> Result<SignedFile, Error> {
        match &self.signed_test {
            Some(test_file) => Ok(test_file.clone()),
            None => {

                let file_path = match &self.test_file_path_str {
                    Some(file_path) => file_path,
                    None => return Err(Error::for_user(Kind::InvalidInput, "Please provide a test file path for the action.".to_string()))
                };
                let file_signature = match &self.test_file_signature_str {
                    Some(file_signature) => file_signature,
                    None => return Err(Error::for_user(Kind::InvalidInput, "Please provide a test file signature for the action.".to_string()))
                };

                let valid_signature = Signature::try_from(file_signature).map_err(|e| Error::for_user(Kind::InvalidInput,
                                                           format!("There is an issue with the test file signature '{}'. {}", file_signature, e.message)))?;
                SignedFile::new(file_path, &valid_signature).map_err(|e| Error::for_user(Kind::InvalidInput,
                                                                         format!("There is an issue with the test file path '{}'. {}", file_path, e.message)))
            }
        }
    }

     fn validate_signed_evidence_file(&self) -> Result<SignedFile, Error> {
        match &self.signed_evidence {
            Some(evidence_file) => Ok(evidence_file.clone()),
            None => {

                let file_path = match &self.evidence_file_path_str {
                    Some(file_path) => file_path,
                    None => return Err(Error::for_user(Kind::InvalidInput, "Please provide an evidence file path for the action.".to_string()))
                };
                let file_signature = match &self.evidence_file_signature_str {
                    Some(file_signature) => file_signature,
                    None => return Err(Error::for_user(Kind::InvalidInput, "Please provide an evidence file signature for the action.".to_string()))
                };

                let valid_signature = Signature::try_from(file_signature).map_err(|e| Error::for_user(Kind::InvalidInput,
                                                           format!("There is an issue with the evidence file signature '{}'. {}", file_signature, e.message)))?;
                SignedFile::new(file_path, &valid_signature).map_err(|e| Error::for_user(Kind::InvalidInput,
                                                                         format!("There is an issue with the evidence file path '{}'. {}", file_path, e.message)))

            }
        }
    }
}
