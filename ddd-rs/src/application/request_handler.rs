use crate::presentation::{self, Request};

/// Trait for representing a **Request Handler**.
///
/// See [Request] for more information about Requests (Commands / Queries).
///
/// # Examples
///
/// ```
/// use ddd_rs::application::RequestHandler;
/// use ddd_rs::presentation::{self, Request};
///
/// #[derive(serde::Deserialize)]
/// struct ListFibonacciQuery {
///     n: u32,
/// }
///
/// impl Request for ListFibonacciQuery {
///     type Response = Vec<u32>;
/// }
///
/// struct FibonacciService;
///
/// impl FibonacciService {
///     fn fibonacci(&self, n: u32) -> u32 {
///         match n {
///             0 => 0,
///             1 => 1,
///             _ => self.fibonacci(n - 1) + self.fibonacci(n - 2),
///         }
///     }
/// }
///
/// #[async_trait::async_trait]
/// impl RequestHandler<ListFibonacciQuery> for FibonacciService {
///     async fn handle(&self, request: ListFibonacciQuery) -> presentation::Result<Vec<u32>> {
///         Ok((0..request.n).map(|n| self.fibonacci(n)).collect())
///     }
/// }
///
/// # tokio_test::block_on(async {
/// let result = FibonacciService.handle(ListFibonacciQuery { n: 10 }).await;
///
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
/// # })
/// ```
#[async_trait::async_trait]
pub trait RequestHandler<T: Request>: Send + Sync {
    /// Handles the incoming [Request], returning its Response as a [Result](presentation::Result).
    async fn handle(&self, request: T) -> presentation::Result<<T as Request>::Response>;
}
