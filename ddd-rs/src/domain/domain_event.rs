/// Trait for representing a **Domain Event**.
///
/// > An event is something that has happened in the past. A domain event is, something that
/// > happened in the domain that you want other parts of the same domain (in-process) to be aware
/// > of.
///
/// # Examples
///
/// The [DomainEvent] trait can be implemented for structs, but also for an enum that discriminates
/// each variant (struct). This is useful to support multiple event types in a single
/// [AggregateRoot].
///
/// Derive its implementation using the [ddd_rs::DomainEvent](crate::DomainEvent) macro:
///
/// ```
/// use ddd_rs::domain::DomainEvent;
///
/// // This is the main `DomainEvent` type: an enum with all possible event variants.
/// #[derive(ddd_rs::DomainEvent)]
/// enum MyDomainEvent {
///     A(ADomainEvent),
///     B(BDomainEvent),
///     C(CDomainEvent),
/// }
///
/// // And these are each variant definition: A, B and C.
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
/// let a = MyDomainEvent::A(ADomainEvent::new(true));
/// let b = MyDomainEvent::B(BDomainEvent::new(42));
/// let c = MyDomainEvent::C(CDomainEvent::new(String::from("foo")));
/// ```
pub trait DomainEvent {
    /// Event unique identifier.
    fn id(&self) -> uuid::Uuid;

    /// Event date (UTC).
    fn at(&self) -> chrono::DateTime<chrono::Utc>;
}

/// Stub for a unit Domain Event, when the aggregate doesn't really need to emit any.
#[derive(Clone, Debug)]
pub struct UnitDomainEvent;

impl DomainEvent for UnitDomainEvent {
    fn id(&self) -> uuid::Uuid {
        Default::default()
    }

    fn at(&self) -> chrono::DateTime<chrono::Utc> {
        Default::default()
    }
}
