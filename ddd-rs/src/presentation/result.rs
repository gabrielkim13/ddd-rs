/// Result type for command / query operations.
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Enum for modeling command / query [Result] errors.
///
/// > Commands change the state of a system but do not return a value.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Request has **invalid input** value.
    #[error("Invalid")]
    Invalid(Vec<ValidationError>),

    /// Requester is not **authenticated** when trying to access a protected resource.
    #[error("Unauthorized")]
    Unauthorized,

    /// Requester is authenticated, but not **authorized**, to access a protected resource.
    #[error("Forbidden")]
    Forbidden,

    /// Resource **doesn't exist**, or **not visible** to the requester.
    #[error("NotFound")]
    NotFound,

    /// Operation execution failed due to an **internal error**.
    ///
    /// Different in nature from the other error variants, which are more "guard-like"; this should
    /// be returned when the operation was actually attempted.
    #[error("Internal: {0}")]
    Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// Validation error
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ValidationError {
    /// String representation of the field to which this validation error applies.
    pub identifier: String,
    /// Message describing the validation error.
    pub error_message: String,
}

impl ValidationError {
    /// Creates a new [ValidationError].
    pub fn new(identifier: impl ToString, error_message: impl ToString) -> Self {
        Self {
            identifier: identifier.to_string(),
            error_message: error_message.to_string(),
        }
    }
}
