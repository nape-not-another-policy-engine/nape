use crate::error::{Error, Kind};
use crate::values::nrn::nrn::NRN;
use crate::values::specification::subject_id::SubjectId;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct  Subject {
    pub nrn: NRN,
    pub id: SubjectId,
}

impl Subject {

    /// #Overview
    ///
    /// Attempts to create a new Subject instance.
    ///
    /// # Arguments
    ///
    /// * `nrn` - A string slice representing the NAPE Resource Name of the Subject.
    /// * `id` - A string slice representing the unique identifier of the Subject.
    ///
    /// # Returns
    ///
    /// A [`Result`] containing either a [`Subject`] instance or an [`Error`].
    pub fn try_new(nrn: &str, id: &str) -> Result<Subject, Error> {

        let validated_nrn = match NRN::new(nrn) {
            Ok(nrn) => nrn,
            Err(e) => return Err(Error::for_user(Kind::InvalidInput, format!("We were unable to create the Subject: {}", e.message))),
        };
        let validated_subject_id = match SubjectId::new(id) {
            Ok(id) => id,
            Err(e) => return Err(Error::for_user(Kind::InvalidInput, format!("We were unable to create the Subject: {}", e.message))),
        };

        Ok(Subject { nrn: validated_nrn, id: validated_subject_id })
    }
}
