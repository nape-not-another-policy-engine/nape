//! Shared bounded, stable local-file admission helpers.

use std::{fs, io::Read, path::Path};

use kernel_oss::error::{Error, Kind};

pub(crate) fn read_regular_stable(path: &Path, maximum_bytes: u64) -> Result<Vec<u8>, Error> {
    let before = fs::symlink_metadata(path)
        .map_err(|_| Error::for_user(Kind::InvalidInput, "local input is unavailable"))?;
    if before.file_type().is_symlink() || !before.is_file() {
        return Err(Error::for_user(
            Kind::InvalidInput,
            "local input must be a regular non-symlink file",
        ));
    }
    if before.len() > maximum_bytes {
        return Err(Error::for_user(
            Kind::ExceedsMax,
            "local input exceeds its effective maximum",
        ));
    }
    let capacity = usize::try_from(before.len()).map_err(|_| {
        Error::for_user(
            Kind::ExceedsMax,
            "local input cannot fit in the bounded reader",
        )
    })?;
    let mut bytes = Vec::with_capacity(capacity);
    fs::File::open(path)
        .and_then(|file| {
            file.take(maximum_bytes.saturating_add(1))
                .read_to_end(&mut bytes)
        })
        .map_err(|_| Error::for_user(Kind::InvalidInput, "local input could not be read"))?;
    if bytes.len() as u64 > maximum_bytes {
        return Err(Error::for_user(
            Kind::ExceedsMax,
            "local input exceeds its effective maximum",
        ));
    }
    let after = fs::symlink_metadata(path)
        .map_err(|_| Error::for_user(Kind::InvalidInput, "local input changed while read"))?;
    if before.len() != after.len()
        || before.modified().ok() != after.modified().ok()
        || !after.is_file()
        || after.file_type().is_symlink()
    {
        return Err(Error::for_user(
            Kind::InvalidInput,
            "local input changed while read",
        ));
    }
    Ok(bytes)
}
