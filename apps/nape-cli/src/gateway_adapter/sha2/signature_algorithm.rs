use sha2::{Sha256, Digest};
use nape_kernel::algorithms::signature_algorithm::{Signature, SignatureType};
use nape_kernel::error::{Error, Kind};

pub fn sha256_signature(data: &Vec<u8>) -> Result<Signature, Error> {

    if data.is_empty() {
        return Err(Error::for_system(Kind::InvalidInput, "Failed to generate a SHA256 signature because the input data you provided is empty.".to_string()));
    }

    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let signature = hex::encode(result);

    Signature::try_new(SignatureType::SHA256, &signature)
}