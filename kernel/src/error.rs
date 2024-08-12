use std::fmt;

/// The error type for any type of operations
///
/// Custom instances of `Error` can be created with crafted error messages and a particular value of [`Kind`].
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error {
    pub audience: Audience,
    pub kind: Kind,
    pub message: String
}
impl Error {
    pub fn new(audience: Audience, kind: Kind, message: String ) -> Error {
        Error { audience, kind, message: String::from(message) }
    }

    pub fn for_user(kind: Kind, message: String) -> Error {
        Error::new(Audience::User, kind, message)
    }

    pub fn for_system(kind: Kind, message: String) -> Error {
        Error::new(Audience::System, kind, message)
    }

    pub fn is_user(&self) -> bool {
        self.audience == Audience::User
    }

    pub fn is_system(&self) -> bool {
        self.audience == Audience::System
    }

}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// A list specifying the audience a of kernel error.
///
/// It is used with the [`kernel::Error`] type.
///
/// [`Kernel::Error`]: Error
///
/// # Handling errors and matching on `Audience`
///
/// In application code, use `match` for the `Audience` values you are expecting.
/// There are only two values, so it is recommended to match on both, one, or the other.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Audience { User, System }

/// A list specifying general categories of kernel errors.
///
/// This list is intended to grow over time, and it is not recommended to
/// exhaustively match against it.
///
/// It is used with the [`kernel::Error`] type.
///
/// [`Kernel::Error`]: Error
///
/// # Handling errors and matching on `Kind`
///
/// In application code, use `match` for the `Kind` values you are
/// expecting; use `_` to match "all other errors"..
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Kind {
    /// A value exceeds the maximum allowed length
    ExceedsMax,
    /// A value is below the minimum allowed length
    BelowMin,
    /// An entity was not found
    NotFound,
    /// The supplied input does not meet the required constaints
    InvalidInput,
    /// Generally used as the default for a match statement when all other Kinds have been exhausted
    Unexpected,
    /// A Gateway execution error occurred when executing a gateway_adapter implementation; This is generally thrown by a Gateway implementation.
    GatewayError,
    /// An Usecase execution error occurred when executing a usecase_configuration; This is generally throw by an Usecase implementation.
    UsecaseError,
    /// A Permission Denied error occurred when a user does not have the required permissions to perform an action.
    PermissionDenied,
    /// When a some type of local (in-memory) data processing failure occurs.
    ProcessingFailure,

}