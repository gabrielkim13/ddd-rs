/// Trait for representing a **Request**.
///
/// Requests are usually **commands** or **queries**, and their sole requirement is to have an
/// associated Response type.
///
/// > - Queries: Return a result and do not change the observable state of the system (are free of
/// >   side effects).
/// > - Commands: Change the state of a system but do not return a value.
///
/// # Examples
///
/// ```
/// use ddd_rs::presentation::Request;
///
/// struct MyRequest {
///     a: bool,
///     b: i32,
///     c: String,
/// }
///
/// struct MyResponse {
///     foo: String,
///     bar: i32,
/// }
///
/// impl Request for MyRequest {
///     type Response = MyResponse;
/// }
/// ```
#[cfg(not(feature = "serde"))]
pub trait Request: Send {
    /// Request response type.
    type Response: Send;
}

#[cfg(feature = "serde")]
pub trait Request: serde::Deserialize<'static> + Send {
    type Response: serde::Serialize + Send;
}
