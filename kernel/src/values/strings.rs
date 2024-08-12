
/// The maximum length of a string.  Use when there needs to be a maximum length for a string for validation.
pub const STRING_256_MAX: usize = 256;

/// Removes all whitespace from a string including between characters as well as leading and trailing whitespace.
pub fn remove_all_whitespace(input: &str) -> String {
    input.replace(" ", "")
}

/// Checks if a string exceeds a maximum length.
pub fn exceeds_max_length(input: &str, max_length: usize) -> bool {
    input.len() > max_length
}

/// Checks if a string is only alphanumeric characters.
pub fn is_only_alphanumeric(input: &str) -> bool {
    input.chars().all(|c| c.is_alphanumeric())
}

/// Checks if a string has more than alphanumeric characters within it.
pub fn has_more_than_alphanumeric(input: &str) -> bool {
    !is_only_alphanumeric(input)
}