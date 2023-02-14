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
        use std::collections::HashMap;

        use axum::Json;
        use http::status::StatusCode;

        match self {
            Error::Invalid(validation_errors) => {
                let value = serde_json::json!({
                    "status_code": 400,
                    "message": "One or more errors occurred!",
                    "errors": validation_errors
                        .into_iter()
                        .map(|ValidationError { identifier, error_message, .. }| (identifier, error_message))
                        .collect::<HashMap<_, _>>(),
                });

                (StatusCode::BAD_REQUEST, Json(value)).into_response()
            }
            Error::Unauthorized => StatusCode::UNAUTHORIZED.into_response(),
            Error::Forbidden => StatusCode::FORBIDDEN.into_response(),
            Error::NotFound => StatusCode::NOT_FOUND.into_response(),
            Error::Internal(e) => {
                let value = serde_json::json!({
                    "status": "Internal Server Error!",
                    "code": 500,
                    "reason": e.to_string(),
                    "note": "See application log for stack trace.",
                });

                (StatusCode::INTERNAL_SERVER_ERROR, Json(value)).into_response()
            }
        }
    }
}
