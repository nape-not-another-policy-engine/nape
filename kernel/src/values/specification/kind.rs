use std::fmt::Display;
use crate::error;
use crate::error::Error;

/// The [`KIND_STRINGS`] contains the list of[`Kind`]s and their string representations.  This is used for validation of inputs  when converting from string to [`Kind`] enum, and when converting from [`Kind`] enum to string.
const KIND_STRINGS: &'static [(Kind, &'static str)] = &[
    (Kind::AssuranceReport, "AssuranceReport"),
    (Kind::AssuranceProcedure, "AssuranceProcedure"), ];

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Kind {
    AssuranceProcedure,
    AssuranceReport,
}

impl Kind {
    pub fn new(kind_value: &str) -> Result<Kind, Error> {
        if kind_value.is_empty() {
            return Err(Error::for_user(error::Kind::InvalidInput, "You have provided an empty Kind value. Please provide a Kind value.".to_string()));
        }
        for (kind_enum, human_readable_kind) in KIND_STRINGS {
            if kind_value == *human_readable_kind { return Ok(kind_enum.clone()); }
        }
        let list_of_valid_kind_inputs: Vec<&str> = KIND_STRINGS.iter().map(|(_, nid_str)| *nid_str).collect();
        Err(Error::for_user(
            crate::error::Kind::InvalidInput,
            format!("'{}' is not a valid Kind. Must be one of: [{}].", kind_value, list_of_valid_kind_inputs.join(", "))
        ) )
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for (kind_enum, kind_str) in KIND_STRINGS {
            if self == kind_enum { return write!(f, "{}", kind_str); }
        })
    }
}
