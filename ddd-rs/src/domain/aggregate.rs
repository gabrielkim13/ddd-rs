/// Trait for representing an **Aggregate Root**.
///
/// > An Aggregate is a group of associated objects which are considered as one unit with regard to
/// > data changes. The Aggregate is demarcated by a boundary which separates the objects inside
/// > from those outside. Each Aggregate has one root. The root is an Entity, and it is the only
/// > object accessible from outside. The root can hold references to any of the aggregate objects,
/// > and the other objects can hold references to each other, but an outside object can hold
/// > references only to the root object. If there are other Entities inside the boundary, the
/// > identity of those entities is local, making sense only inside the aggregate.
///
/// # Example
///
/// Derive its implementation using the [ddd_rs::AggregateRoot](crate::AggregateRoot) macro:
///
/// ```
/// // The `AggregateRoot` usually holds references to other entities, acting as a means to access
/// // and even modify them.
/// //
/// // Note that we also need to derive the `Entity` trait, since an `AggregateRoot` is an `Entity`.
/// #[derive(ddd_rs::AggregateRoot, ddd_rs::Entity)]
/// struct MyAggregateRoot {
///     #[entity(id)]
///     id: u32,
///     foo: Foo,
///     bars: Vec<Bar>,
/// }
///
/// #[derive(ddd_rs::Entity)]
/// struct Foo {
///     #[entity(id)]
///     id: u32,
///     foo: String,
/// }
///
/// #[derive(ddd_rs::Entity)]
/// struct Bar {
///     #[entity(id)]
///     id: String,
///     bar: u32,
/// }
/// ```
pub trait AggregateRoot: super::Entity + Send + Sync + 'static {}

/// Extensions to the [AggregateRoot] behavior.
///
/// # Example
///
/// ```
/// use std::sync::Mutex;
///
/// use ddd_rs::domain::AggregateRootEx;
///
/// // The `DomainEvent` will usually be an arithmetic enum type, in order to allow for multiple
/// // distinguishable event kinds within a single type.
/// #[derive(Debug, PartialEq)]
/// enum MyDomainEvent {
///     DidSomething { something: String },
///     DidSomethingElse { something_else: String },
/// }
///
/// // The `AggregateRoot` owns a list of its own `DomainEvent`s.
/// //
/// // The [Interior Mutability](https://doc.rust-lang.org/reference/interior-mutability.html)
/// // pattern may be relevant when semantically immutable actions need to register domain events.
/// #[derive(ddd_rs::AggregateRoot, ddd_rs::Entity)]
/// struct MyAggregateRoot {
///     #[entity(id)]
///     id: u32,
///     domain_events: Mutex<Vec<MyDomainEvent>>,
/// }
///
/// // The aggregate root's methods may register domain events upon different actions.
/// impl MyAggregateRoot {
///     pub fn new(id: u32) -> Self {
///         Self {
///             id,
///             domain_events: Default::default(),
///         }
///     }
///
///     pub fn do_something(&self, something: impl ToString) {
///         let something = something.to_string();
///
///         // Do something...
///
///         self.register_domain_event(MyDomainEvent::DidSomething { something });
///     }
///
///     pub fn do_something_else(&mut self, something_else: impl ToString) {
///         let something_else = something_else.to_string();
///
///         // Do something else...
///
///         self.register_domain_event(MyDomainEvent::DidSomethingElse { something_else });
///     }
///
///     fn register_domain_event(&self, domain_event: <Self as AggregateRootEx>::DomainEvent) {
///         let mut domain_events = self.domain_events.lock().unwrap();
///
///         domain_events.push(domain_event);
///     }
/// }
///
/// impl AggregateRootEx for MyAggregateRoot {
///     type DomainEvent = MyDomainEvent;
///
///     fn take_domain_events(&mut self) -> Vec<Self::DomainEvent> {
///         let mut domain_events = self.domain_events.lock().unwrap();
///
///         domain_events.drain(..).collect()
///     }
/// }
///
/// let aggregate_root = MyAggregateRoot::new(42);
///
/// // This registers a `MyDomainEvent::DidSomething` event.
/// //
/// // Note that this happens under an immutable reference to the aggregate.
/// aggregate_root.do_something("foo");
///
/// let mut aggregate_root = aggregate_root;
///
/// // This registers a `MyDomainEvent::DidSomethingElse` event.
/// aggregate_root.do_something_else("bar");
///
/// // Take the domain events and assert that they are gone afterwards.
/// let domain_events = aggregate_root.take_domain_events();
///
/// assert_eq!(
///     domain_events[0],
///     MyDomainEvent::DidSomething {
///         something: "foo".to_string()
///     }
/// );
/// assert_eq!(
///     domain_events[1],
///     MyDomainEvent::DidSomethingElse {
///         something_else: "bar".to_string()
///     }
/// );
///
/// assert!(aggregate_root.take_domain_events().is_empty());
/// ```
pub trait AggregateRootEx: AggregateRoot {
    /// Domain event type.
    ///
    /// > Use domain events to explicitly implement side effects of changes within your domain.
    /// > In other words, and using DDD terminology, use domain events to explicitly implement side
    /// > effects across multiple aggregates.
    type DomainEvent: Send;

    /// Clears all domain events from the aggregate, returning them in order of occurrence.
    fn take_domain_events(&mut self) -> Vec<Self::DomainEvent>;
}
