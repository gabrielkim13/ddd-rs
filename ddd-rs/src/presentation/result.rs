use crate::BoxError;

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
    Internal(#[source] BoxError),
}

#[cfg(feature = "serde")]
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Error", 2)?;

        match self {
            Error::Invalid(validation_errors) => {
                state.serialize_field("message", "Bad request")?;
                state.serialize_field("errors", validation_errors)?;
            }
            Error::Unauthorized => {
                state.serialize_field("message", "Unauthorized")?;
                state.serialize_field::<[()]>("errors", &[])?;
            }
            Error::Forbidden => {
                state.serialize_field("message", "Forbidden")?;
                state.serialize_field::<[()]>("errors", &[])?;
            }
            Error::NotFound => {
                state.serialize_field("message", "Not found")?;
                state.serialize_field::<[()]>("errors", &[])?;
            }
            Error::Internal(e) => {
                state.serialize_field("message", "Internal server error")?;
                state.serialize_field("errors", &[e.to_string()])?;
            }
        }

        state.end()
    }
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
    pub fn new(identifier: impl Into<String>, error_message: impl ToString) -> Self {
        Self {
            identifier: identifier.into(),
            error_message: error_message.to_string(),
        }
    }
}

impl From<BoxError> for Error {
    fn from(err: BoxError) -> Self {
        Self::Internal(err)
    }
}

#[cfg(feature = "axum")]
impl axum_core::response::IntoResponse for Error {
    fn into_response(self) -> axum_core::response::Response {
        use axum::Json;
        use http::status::StatusCode;

        match &self {
            Error::Invalid(_) => (StatusCode::BAD_REQUEST, Json(self)).into_response(),
            Error::Unauthorized => StatusCode::UNAUTHORIZED.into_response(),
            Error::Forbidden => StatusCode::FORBIDDEN.into_response(),
            Error::NotFound => StatusCode::NOT_FOUND.into_response(),
            Error::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response(),
        }
    }
}
