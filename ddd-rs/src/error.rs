/// Alias for a type-erased error type.
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// `Result` type with a pre-defined [BoxError] error variant.
pub type Result<T, E = BoxError> = std::result::Result<T, E>;
