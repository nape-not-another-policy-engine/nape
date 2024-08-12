use std::fmt::{Display};
use std::string::ToString;
use crate::error::{Error, Kind};

pub const NRN_SCHEME: &str = "nrn";

/// The [`ALLOWED_NRN_NID_LIST`] contains the list of[`NID`]s that are allowed for an [`NRN`] and their human-readable string representations.
pub const ALLOWED_NRN_NID_LIST: &'static [(NapeNID, &'static str)] = &[
    (NapeNID::Procedure, "procedure"),
    (NapeNID::SourceCode, "sourcecode"), ];

/// An NAPE Resource Name (NRN) is a specific version of a Universal Resource Name (URN) that identifies a resource within the NAPE ecosystem. It is in the format `nrn:<nid>:<nss>`. The [`NRN`] struct contains the following fields:
///
/// * `value` - The full [`NRN`] string
/// * `scheme` - The scheme of the [`NRN`]. This should always be `nrn`
/// * `nid` - The Namespace Identifier [`NID`] of the [`NRN`]
/// * `nss` - The Namespace String [`NSS`] of the [`NRN`]
///
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NRN {
     pub value: String,
     pub scheme: String,
     pub nid: NapeNID,
     pub nss: Vec<NSS>,
}

impl NRN {

    /// Create a new NRN instance
    ///
    /// # Arguments
    ///
    /// * `nrn` - A string slice containing the NRN value
    ///
    /// # Returns
    ///
    /// * `Result<NRN, Error>` - A result containing either an error or a new NRN instance.
    ///
    /// An [`Error`]  of [`Kind::InvalidInput`]  is returned if:
    ///  * the input is empty,
    /// * does not start with the correct scheme,
    /// * contains an invalid [`NID`],
    /// * the [`NID`] provided is not an [`NapeNID`] within the [`ALLOWED_NRN_NID_LIST`], or
    /// * does not contain at least one [`NSS`].
    ///
    pub fn new(nrn: &str) -> Result<NRN, Error> {

        Self::check_nrn_is_not_empty(nrn)?;
        Self::check_nrn_starts_with_correct_scheme(nrn)?;
        let _nid = Self::check_for_valid_nid_and_extract(nrn)?;
        let _nss = Self::check_for_valid_nss_and_extract(nrn)?;

        Ok(NRN {
            value: String::from(nrn),
            scheme: String::from(NRN_SCHEME),
            nid: _nid,
            nss: _nss
        })
    }
    fn check_nrn_is_not_empty(nrn: &str) -> Result<(), Error> {
        if nrn.is_empty() {
            return Err(Error::for_user(Kind::InvalidInput, "You provided an empty NRN value. An NRN must be in the format 'nrn:<nid>:<nss>'".to_string()));
        }
        Ok(())
    }
    fn check_nrn_starts_with_correct_scheme(nrn: &str) -> Result<(), Error> {
        let scheme = nrn.split(":").next().unwrap();
        if scheme != NRN_SCHEME {
            return Err(Error::for_user(Kind::InvalidInput, format!("You provided '{}' as an NRN and the scheme '{}'  is not valid. Must be '{}'", nrn, scheme, NRN_SCHEME)));
        }
        Ok(())
    }
    fn check_for_valid_nid_and_extract(nrn: &str) -> Result<NapeNID, Error> {
        let extracted_nid = nrn.split(":").nth(1).unwrap();
        let nid = NID::new(extracted_nid)?;
        NapeNID::new(nid)
    }
    fn check_for_valid_nss_and_extract(nrn: &str) -> Result<Vec<NSS>, Error>  {

        let parts: Vec<&str> = nrn.split(":").collect();
        if parts.len() < 3 {
            return Err(Error::for_user(Kind::InvalidInput, format!("The supplied NRN '{}' must have at least one NSS after the NID.",nrn)));
        }

    let nss_parts = &parts[2..];
    let mut nss_vec = Vec::new();
    for nss in nss_parts {
        let nss = NSS::new(nss)?;
        nss_vec.push(nss); }

    Ok(nss_vec)

    }

}

/// An [`NapeNID`] is an enumeration of the different types of resources that can be identified within an [`NRN`] namespace.  The [`ALLOWED_NRN_NID_LIST`] contains the list of [`NID`]s that are allowed.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum NapeNID {
    Procedure,
    SourceCode
}
impl NapeNID {

    pub fn new(nid: NID) -> Result<NapeNID, Error> {
        let nid_value = nid.value.as_str();
        for (nid_enum, human_readable_nid) in ALLOWED_NRN_NID_LIST {
            if nid_value == *human_readable_nid { return Ok(nid_enum.clone()); }
        }
        let list_of_valid_nid_inputs: Vec<&str> = ALLOWED_NRN_NID_LIST.iter().map(|(_, nid_str)| *nid_str).collect();
        Err(Error::for_user(
            Kind::InvalidInput,
            format!("'{}' is not a valid NID. Must be one of: [{}].", nid_value, list_of_valid_nid_inputs.join(", "))
        ) )
    }

}
impl Display for NapeNID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for (nid, nid_str) in ALLOWED_NRN_NID_LIST {
            if self == nid { return write!(f, "{}", nid_str); }
        })
    }
}

/// A Namespace Identifier (NID) is an enumeration of the different types of resources that can be identified within a namespace.
pub struct NID {
    pub value: String
}
impl NID {
    pub fn new(nid_input: &str) -> Result<NID, Error> {

        if nid_input.is_empty() {
            return Err(Error::for_user(Kind::InvalidInput, "The NID provided is blank.".to_string()));
        }

        let first_char = nid_input.chars().next().unwrap();
        if !first_char.is_alphabetic() {
            return Err(Error::for_user(Kind::InvalidInput, format!("The NID must start with an alphabetic character. You NID '{}' starts with '{}'.", nid_input, first_char)))
        }

        for (index, character) in nid_input.chars().enumerate() {
            if !character.is_alphanumeric() && character != '-' {
                return Err(Error::for_user(Kind::InvalidInput, format!("NID '{}' contains invalid character '{}' at position {}.  An NID can only contains alphanumeric characters or a dash '-'.", nid_input, character, index)));
            }
        }

        if nid_input.len() > 31 {
            return Err(Error::for_user(Kind::InvalidInput, format!("The NID '{}' is too long. An NID must be 31 characters or less.", nid_input)));
        }

        Ok(NID { value: String::from(nid_input) })
    }
}
impl Display for NID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// A Namespace String (NSS) is an ASCII compliant string, which contains no whitespace, that is used to identify a resource within a namespace.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NSS {
    pub value: String
}
impl NSS {

    /// Create a new NSS instance
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice containing the namespace string
    ///
    /// # Returns
    ///
    /// * `Result<NSS, Error>` - A result containing either an error or a new NSS instance. An [`Error`]  of [`Kind::InvalidInput`]  is returned if the input contains non-ASCII characters or whitespaces.
    ///
    pub fn new(input: &str) -> Result<NSS, Error> {
        for (index, character) in input.chars().enumerate() {
            if !character.is_ascii() {
                return Err(Error::for_user(Kind::InvalidInput, format!("Input '{}' contains non-ASCII character '{}' at position {}", input, character, index)));
            }
            if character.is_whitespace() {
                return Err(Error::for_user(Kind::InvalidInput, format!("Input '{}' contains whitespace character '{}' at position {}", input, character, index)));
            }
        }
        Ok(NSS { value: String::from(input) })
    }
}