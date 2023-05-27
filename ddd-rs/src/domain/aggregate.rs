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
