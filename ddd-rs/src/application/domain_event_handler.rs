use crate::domain::DomainEvent;

/// Result type for [DomainEventHandler] operations.
pub type Result<T, E = Box<dyn std::error::Error + Send + Sync>> = core::result::Result<T, E>;

/// Trait for representing a **Domain Event Handler**.
///
/// See [DomainEvent] for more information about Domain Events.
///
/// # Examples
///
/// Implement the [DomainEventHandler] trait for each [DomainEvent] variant (struct). Then, use the
/// [ddd_rs::DomainEvent](crate::DomainEvent) macro, and the `handler` attribute to derive the trait
/// implementation for the main [DomainEvent] enum:
///
/// ```
/// use ddd_rs::application::domain_event_handler::{self, DomainEventHandler};
/// use ddd_rs::domain::DomainEvent;
///
/// #[derive(ddd_rs::DomainEvent)]
/// #[domain_event(handler = "MyDomainEventHandler")]
/// enum MyDomainEvent {
///     A(ADomainEvent),
///     B(BDomainEvent),
///     C(CDomainEvent),
/// }
///
/// #[derive(ddd_rs::DomainEvent)]
/// struct ADomainEvent {
///     id: uuid::Uuid,
///     a_field: bool,
///     at: chrono::DateTime<chrono::Utc>,
/// }
///
/// #[derive(ddd_rs::DomainEvent)]
/// struct BDomainEvent {
///     id: uuid::Uuid,
///     b_field: i32,
///     at: chrono::DateTime<chrono::Utc>,
/// }
///
/// #[derive(ddd_rs::DomainEvent)]
/// struct CDomainEvent {
///     id: uuid::Uuid,
///     c_field: String,
///     at: chrono::DateTime<chrono::Utc>,
/// }
///
/// struct MyDomainEventHandler;
///
/// #[async_trait::async_trait]
/// impl DomainEventHandler<ADomainEvent> for MyDomainEventHandler {
///     async fn handle(&self, event: ADomainEvent) -> domain_event_handler::Result<()> {
///         match event.a_field {
///             true => Ok(()),
///             _ => Err("a_field is not true".into()),
///         }
///     }
/// }
///
/// #[async_trait::async_trait]
/// impl DomainEventHandler<BDomainEvent> for MyDomainEventHandler {
///     async fn handle(&self, event: BDomainEvent) -> domain_event_handler::Result<()> {
///         match event.b_field {
///             1 => Ok(()),
///             _ => Err("b_field is not 1".into()),
///         }
///     }
/// }
///
/// #[async_trait::async_trait]
/// impl DomainEventHandler<CDomainEvent> for MyDomainEventHandler {
///     async fn handle(&self, event: CDomainEvent) -> domain_event_handler::Result<()> {
///         match event.c_field.as_str() {
///             "1" => Ok(()),
///             _ => Err("c_field is not \"1\"".into()),
///         }
///     }
/// }
///
/// # tokio_test::block_on(async {
/// let a = MyDomainEvent::A(ADomainEvent::new(true));
/// let b = MyDomainEvent::B(BDomainEvent::new(42));
/// let c = MyDomainEvent::C(CDomainEvent::new(String::from("1")));
///
/// assert!(MyDomainEventHandler.handle(a).await.is_ok());
/// assert!(MyDomainEventHandler.handle(b).await.is_err());
/// assert!(MyDomainEventHandler.handle(c).await.is_ok());
/// # })
/// ```
#[async_trait::async_trait]
pub trait DomainEventHandler<T: DomainEvent>: Send + Sync {
    /// Handles the incoming [DomainEvent], a unit [Result].
    async fn handle(&self, event: T) -> Result<()>;
}
