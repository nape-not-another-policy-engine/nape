use std::hash::{Hash};
use crate::algorithms::signature_algorithm::{Signature, SignatureType};
use crate::error::Error;
use crate::values::specification::file_path::FilePath;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SignedFile {
    file: FilePath,
    signature: Signature,

}

impl SignedFile {

    pub fn new(file_location: &str, signature: &Signature) -> Result<Self, Error> {
        let file_path = FilePath::try_from(file_location)?;
       Ok( Self {
            file: file_path,
            signature: signature.clone(),
        })
    }

    /// Get a reference to the file path
    pub fn file(&self) -> &FilePath { &self.file }

    /// Get a reference to the signature
    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    /// Get a reference to the signature type
    pub fn signature_type(&self) -> &SignatureType {
        self.signature.signature_type()
    }

}


