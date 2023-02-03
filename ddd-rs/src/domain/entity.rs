/// Trait for representing an **Entity**.
///
/// > There is a category of objects which seem to have an identity, which remains the same
/// > throughout the states of the software. For these objects it is not the attributes which
/// > matter, but a thread of continuity and identity, which spans the life of a system and can
/// > extend beyond it. Such objects are called Entities.
///
/// # Examples
///
/// Derive its implementation using the [ddd_rs::Entity](crate::Entity) macro:
///
/// ```
/// use ddd_rs::domain::Entity;
///
/// #[derive(ddd_rs::Entity, Debug)]
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
///
/// let a = MyEntity::new(1, "foo");
/// let b = MyEntity::new(1, "bar");
/// let c = MyEntity::new(2, "foo");
///
/// assert_eq!(a, b);
/// assert_ne!(a, c);
/// ```
pub trait Entity: Eq + PartialEq {
    /// Identity type.
    type Id: Clone + Copy + PartialEq + Send + Sync;

    /// Identity.
    fn id(&self) -> Self::Id;

    /// Creation date (UTC).
    fn created_at(&self) -> &chrono::DateTime<chrono::Utc>;

    /// Last update date (UTC).
    fn updated_at(&self) -> &chrono::DateTime<chrono::Utc>;
}
