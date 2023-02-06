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
/// # Examples
///
/// Derive its implementation using the [ddd_rs::AggregateRoot](crate::AggregateRoot) macro:
///
/// ```
/// use ddd_rs::domain::{AggregateRoot, Entity};
///
/// #[derive(ddd_rs::AggregateRoot, ddd_rs::Entity)]
/// struct MyEntity {
///     id: i32,
///     my_field: String,
///     created_at: chrono::DateTime<chrono::Utc>,
///     updated_at: chrono::DateTime<chrono::Utc>,
/// }
///
/// impl MyEntity {
///     pub fn new(id: i32, my_field: impl ToString) -> Self {
///         Self {
///             id,
///             my_field: my_field.to_string(),
///             created_at: chrono::Utc::now(),
///             updated_at: chrono::Utc::now(),
///         }
///     }
/// }
/// ```
pub trait AggregateRoot: super::Entity + Send + Sync + 'static {}
