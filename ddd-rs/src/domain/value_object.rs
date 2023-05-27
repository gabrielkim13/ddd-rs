/// Trait for representing a **Value Object**.
///
/// > There are cases when we need to contain some attributes of a domain element. We are not
/// > interested in which object it is, but what attributes it has. An object that is used to
/// > describe certain aspects of a domain, and which does not have identity, is named Value Object.
///
/// # Examples
///
/// Derive its implementation using the [ddd_rs::ValueObject](crate::ValueObject) macro:
///
/// ```
/// // Annotate the equality components with the `#[value_object(eq)]` attribute.
/// #[derive(ddd_rs::ValueObject, Debug)]
/// struct MyValueObject {
///     #[value_object(eq)]
///     x: bool,
///     y: Option<i32>,
///     #[value_object(eq)]
///     z: String,
/// }
///
/// let a = MyValueObject { x: true, y: Some(42), z: String::from("foo") };
/// let b = MyValueObject { x: true, y: Some(-1), z: String::from("foo") };
/// let c = MyValueObject { x: false, y: Some(42), z: String::from("bar") };
///
/// // `ValueObject`s are equal if all their equality components are equal.
/// assert_eq!(a, b);
/// assert_ne!(a, c);
///
/// // They are also cloneable, by definition.
/// let a_clone = a.clone();
///
/// assert_eq!(a.x, a_clone.x);
/// assert_eq!(a.y, a_clone.y);
/// assert_eq!(a.z, a_clone.z);
/// ```
pub trait ValueObject: Clone + PartialEq {}
