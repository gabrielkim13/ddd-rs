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
/// // Annotate the identity field with the `#[entity(id)]` attribute.
/// #[derive(ddd_rs::Entity, Debug)]
/// struct MyEntity {
///     #[entity(id)]
///     code: u32,
///     my_field: String,
/// }
///
/// impl MyEntity {
///     pub fn new(code: u32, my_field: impl ToString) -> Self {
///         Self {
///             code,
///             my_field: my_field.to_string(),
///         }
///     }
/// }
///
/// let a = MyEntity::new(1, "foo");
/// let b = MyEntity::new(1, "bar");
/// let c = MyEntity::new(2, "foo");
///
/// // By definition, `Entity` equality is based exclusively on their identity.
/// assert_eq!(a, b);
/// assert_eq!(a.id(), b.id());
///
/// assert_ne!(a, c);
/// assert_ne!(a.id(), c.id());
/// ```
pub trait Entity: Eq + PartialEq {
    /// Identity type.
    type Id: Clone + PartialEq + Send + Sync;

    /// Identity.
    fn id(&self) -> &Self::Id;
}
