use kernel_oss::algorithms::signature_algorithm::{Signature, SignatureType};
use kernel_oss::error::{Error, Kind};
use sha2::{Digest, Sha256};

pub fn sha256_signature(data: &Vec<u8>) -> Result<Signature, Error> {
    if data.is_empty() {
        return Err(Error::for_system(
            Kind::InvalidInput,
            "Failed to generate a SHA256 signature because the input data you provided is empty."
                .to_string(),
        ));
    }

    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let signature = hex::encode(result);

    Signature::try_new(SignatureType::SHA256, &signature)
}
