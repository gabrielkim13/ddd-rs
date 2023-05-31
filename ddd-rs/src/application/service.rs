use crate::domain::AggregateRootEx;

/// Trait for representing a **Request**.
///
/// Requests are usually [Commands](Command) or [Queries](Query), and their sole requirement is to
/// have an associated `Response` type.
pub trait Request: Send {
    /// Request response type.
    type Response: Send;
}

/// Trait for a [Request] that represents a **Command**.
///
/// > Change the state of a system but do not return a value.
pub trait Command: Send {}

impl<T: Command> Request for T {
    type Response = ();
}

/// Alias for a [Request] that represents a **Query**.
///
/// > Return a result and do not change the observable state of the system (are free of side
/// > effects).
pub use Request as Query;

/// Trait for representing a **Request Handler**.
///
/// See [Request] for more information about [Commands](Command) and [Queries](Query).
///
/// # Examples
///
/// ```
/// use std::sync::Mutex;
///
/// use ddd_rs::application::{Command, CommandHandler, Query, QueryHandler};
///
/// // Error type for the Fibonacci service.
/// //
/// // It is required to implement the `std::error::Error` trait.
///
/// #[derive(Debug, PartialEq)]
/// struct FibonacciError(&'static str);
///
/// impl std::fmt::Display for FibonacciError {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         self.0.fmt(f)
///     }
/// }
///
/// impl std::error::Error for FibonacciError {}
///
/// // Fibonacci service.
/// //
/// // For demonstration purposes, it is implemented as a stateful service.
///
/// #[derive(Default)]
/// struct FibonacciService {
///     n: Mutex<u32>,
/// }
///
/// impl FibonacciService {
///     fn set(&self, n: u32) {
///         let mut current = self.n.lock().unwrap();
///
///         *current = n;
///     }
///
///     fn get_next(&self) -> u32 {
///         let mut current = self.n.lock().unwrap();
///
///         let next = Self::fibonacci(*current);
///
///         *current = *current + 1;
///
///         next
///     }
///
///     fn fibonacci(n: u32) -> u32 {
///         match n {
///             0 => 0,
///             1 => 1,
///             _ => Self::fibonacci(n - 1) + Self::fibonacci(n - 2),
///         }
///     }
/// }
///
/// // Sets the current 0-based element of the Fibonacci sequence.
/// //
/// // `Command`s usually do not return a value, so their `Response` type is automatically `()`.
///
/// struct SetFibonacciCommand {
///     n: u32,
/// }
///
/// impl Command for SetFibonacciCommand {}
///
/// #[async_trait::async_trait]
/// impl CommandHandler<SetFibonacciCommand> for FibonacciService {
///     type Error = FibonacciError;
///
///     async fn handle(&self, command: SetFibonacciCommand) -> Result<(), Self::Error> {
///         self.set(command.n);
///
///         Ok(())
///     }
/// }
///
/// // Gets the next element of the Fibonacci sequence.
/// //
/// // `Query`s are issued in order to retrieve a value, but without causing any side-effects to the
/// // underlying state of the system.
/// //
/// // The more general `Request` trait can be used for actions that have side-effects but also
/// // require a value to be returned as its result.
///
/// struct GetNextFibonacciQuery;
///
/// impl Query for GetNextFibonacciQuery {
///     type Response = u32;
/// }
///
/// #[async_trait::async_trait]
/// impl QueryHandler<GetNextFibonacciQuery> for FibonacciService {
///     type Error = FibonacciError;
///
///     async fn handle(&self, _query: GetNextFibonacciQuery) -> Result<u32, Self::Error> {
///         Ok(self.get_next())
///     }
/// }
///
/// // Finally, instantiate and perform `Request`s to the `FibonacciService`.
///
/// # tokio_test::block_on(async {
/// let fibonacci = FibonacciService::default();
///
/// assert_eq!(fibonacci.handle(SetFibonacciCommand { n: 10 }).await, Ok(()));
/// assert_eq!(fibonacci.handle(GetNextFibonacciQuery).await, Ok(55));
/// assert_eq!(fibonacci.handle(GetNextFibonacciQuery).await, Ok(89));
/// assert_eq!(fibonacci.handle(GetNextFibonacciQuery).await, Ok(144));
/// # })
/// ```
#[async_trait::async_trait]
pub trait RequestHandler<T: Request>: Send + Sync {
    /// Request handler error type.
    type Error: std::error::Error;

    /// Handles the incoming [Request], returning its [Response](Request::Response) as a [Result].
    async fn handle(&self, request: T) -> Result<<T as Request>::Response, Self::Error>;
}

/// Alias for a [RequestHandler] specific to [Commands](Command).
pub use RequestHandler as CommandHandler;

/// Alias for a [RequestHandler] specific to [Queries](Query).
pub use RequestHandler as QueryHandler;

/// Trait for representing a **Domain Event Handler**.
///
/// See [AggregateRootEx] for more information about **Domain Events** and the example for
/// [RepositoryEx](super::RepositoryEx) about this trait's usage.
#[async_trait::async_trait]
pub trait DomainEventHandler<T: AggregateRootEx>: Send + Sync {
    /// Handles the incoming domain event, applying any necessary changes to the entity.
    async fn handle(&self, entity: &mut T, event: T::DomainEvent) -> crate::Result<()>;
}
