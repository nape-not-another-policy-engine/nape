use std::fmt::{Display, Formatter};
use crate::error::{Error, Kind};

/// The [`SIGNATURE_TYPES`] contains the list of signature algorithms that are supported by the system and their human-readable string representations.
pub const SIGNATURE_TYPES: &'static [(SignatureType, &'static str)] = &[
    (SignatureType::SHA256, "SHA256"), ];

/// The [`SignatureAlgorithm`] defines a function type that takes a byte sequence and generates a cryptographic signature of the data. This is generally used when creating a cryptographic signature for a file, although it can be for any data.
///
/// # Arguments
///
/// * `&Vec<u8>` - A reference to a byte sequence which represents the data you want to sign.
///
/// # Returns
///
/// * `Result<Signature, Error>` - A result containing either a [`Signature`] or an [`Error`].
///
/// ## Errors
///
/// All errors returned from the function must be for the [`Audience::System`] and below are the two [`Kind`]s of errors that can be returned:
///  *  [`Kind::InvalidInput`] - The `&Vec<u8>` argument is an empty vector.
///  * [`Kind::ProcessingFailure`] - The signature algorithm failed to sign the data.
///
pub type SignatureAlgorithm = fn(&Vec<u8>) -> Result<Signature, Error>;

/// The [`SignatureType`] enum defines the cryptographic signature algorithms that can be used to sign a file.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SignatureType {  SHA256 }

impl SignatureType {

    pub fn from(algo: &str) -> Result<SignatureType, Error> {
        for (algo_enum, human_readable_algo) in SIGNATURE_TYPES {
            if algo == *human_readable_algo { return Ok(algo_enum.clone()) }
        }
        let list_of_valid_algo: Vec<&str> = SIGNATURE_TYPES.iter().map(|(_, algo)| *algo).collect();
        Err(
            Error::for_system(
                Kind::InvalidInput,
                format!("The signature algorithm '{}' is not supported. The supported algorithms are: [{}]", algo, list_of_valid_algo.join(", ") )
            ) )
    }
}

impl Display for SignatureType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(for (algo, algo_str) in SIGNATURE_TYPES {
            if self == algo { return write!(f, "{}", algo_str); }
        })
    }
}



/// The [`Signature`] struct represents a cryptographic signature of a file and the algorithm used to generate the signature.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Signature {
    signature_type: SignatureType,
    signature: String,
}

impl Signature {

    /// Create a new [`Signature`] instance.
    ///
    /// # Arguments
    ///
    /// * `algorithm` - The [`SignatureType`] used to generate the signature.
    /// * `signature` - The cryptographic signature of the data.
    ///
    /// # Returns
    ///
    /// * `Result<FileSignature, Error>` - A result containing either a new [`Signature`] instance or an [`Error`].
    ///
    /// ## Errors
    ///
    /// An error is returned if the signature is an empty string. and this error  is for the [`Audience::System`] and is of [`Kind::InvalidInput`].
    ///
    pub fn try_new(sig_type: SignatureType, signature: &str) -> Result<Signature, Error> {
        if signature.is_empty() {
            return Err(Error::for_system(Kind::InvalidInput, "The signature you provided is empty. Please provide a non-empty signature value.".to_string()))
        };
        Ok(Self { signature_type: sig_type, signature: signature.to_string()})
    }

    /// Create a new [`Signature`] instance from a string representation of the signature that is stated as ALGO[signature]
    pub fn try_from(signature_input: &str) -> Result<Signature, Error> {
        let signature = extract_and_validate_signature(signature_input)?;
        let signature_type = extract_and_validate_signature_type(signature_input)?;
        Self::try_new(signature_type, &signature)

    }

    /// Get a clone of the signature string
    pub fn to_string(&self) -> String { self.signature.clone() }

    // Get a reference to the signature string
    pub fn as_str(&self) -> &str { &self.signature }

    /// Get a clone of the signature string for the structure signature
    ///
    /// The structure signature is the signature string that is in the format of ALGO[signature]
    pub fn structure_signature(&self) -> String {  format!("{}[{}]", self.signature_type, self.signature) }

    /// Get a clone for the string value for the signature algorithm
    pub fn signature_type(&self) -> &SignatureType { &self.signature_type }

}

pub fn extract_and_validate_signature_type(signature_input: &str) -> Result<SignatureType, Error> {
    let first_bracket = signature_input.find('[').ok_or(
        Error::for_system(Kind::InvalidInput, format!("The signature '{}' is not in the correct format, the first bracket is missing. The correct format is ALGO[signature-data] where ALGO is the signature algorithm (for example SHA256), and 'signature-data' is the signature itself.", signature_input))
    )?;
    let algo_str = signature_input[..first_bracket].to_string();
    if algo_str.is_empty() {
        return Err(Error::for_system(Kind::InvalidInput, "The signature string provided does not have a signature algorithm. The signature algorithm are the characters before the first bracket: ALGO[the-signature-data-here].".to_string()));
    }
    let valid_signature_type = SignatureType::from(&algo_str)?;
    Ok(valid_signature_type)
}

fn extract_and_validate_signature(signature_input: &str) -> Result<String, Error> {
    let start = signature_input.find('[')
        .ok_or(Error::for_system(Kind::InvalidInput,
            format!("The signature '{}' is not in the correct format, the first bracket is missing. Check the signature you provided to ensure it in the format of ALGO[the-signature-data].", signature_input)))? + 1;
    let end = signature_input.find(']')
        .ok_or(Error::for_system(Kind::InvalidInput,
            format!("The signature '{}' is not in the correct format, the last bracket is missing. Check the signature you provided to ensure it in the format of ALGO[the-signature-data].", signature_input)))?;
    if start > end {
        return Err(Error::for_system(Kind::InvalidInput,
            "The signature you provided has the closing bracket before the opening bracket. Check the signature you provided to ensure it in the format of ALGO[the-signature-data].".to_string()))
    }
    let extracted_signature = signature_input[start..end].to_string();
    if extracted_signature.is_empty() {
        return Err(Error::for_system(Kind::InvalidInput, "The signature string provided does not have any signature data. Signature data are the characters between the two brackets: ALGO[the-signature-data-here].".to_string()))
    }
    Ok(extracted_signature)
}

// Design decision
// Passing a File object and passing a Vec<u8> that represents the file data are two different approaches, each with its own advantages and disadvantages.
//
// Passing a File object: This approach is more memory efficient as it doesn't require loading the entire file into memory at once. The file data can be read and processed in chunks. However, it requires the file to be open during the operation, which might not be desirable in some cases. Also, error handling can be more complex because you have to deal with potential file I/O errors.
//
// Passing a Vec<u8>: This approach requires the entire file to be loaded into memory, which might not be feasible for very large files. However, it can be more convenient because you don't have to deal with file I/O during the operation. Once you have the Vec<u8>, you can pass it around, clone it, etc., without worrying about the file itself.
//
// In the context of your FileSignatureAlgorithm function, if you change it to take a Vec<u8>, you would be able to compute the signature of any byte sequence, not just files. This could be useful if you want to compute signatures of data that is not stored in a file. However, you would lose the ability to process the file in chunks, which could be a problem for large files.
//
// Chose Vec<u8> because the file needs to be loaded inot a vectory becfore it can be passed into the signature algorithm anyways, so removing the commplications of maanging the filoe IO from the FileSignatureAlgorithm and focusimg it on only appliyng the signature algorithm to the file data.
