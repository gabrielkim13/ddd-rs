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
/// use ddd_rs::domain::ValueObject;
///
/// #[derive(ddd_rs::ValueObject, Debug)]
/// struct MyNamedValueObject {
///     #[eq_component]
///     x: bool,
///     y: Option<i32>,
///     #[eq_component]
///     z: String,
/// }
///
/// let a = MyNamedValueObject { x: true, y: Some(42), z: String::from("foo") };
/// let b = MyNamedValueObject { x: true, y: Some(-1), z: String::from("foo") };
/// let c = MyNamedValueObject { x: false, y: Some(42), z: String::from("bar") };
///
/// assert_eq!(a, b);
/// assert_ne!(a, c);
///
/// // Comparison by equality components, in order of declaration
/// assert!(a > c); // Because "foo" > "bar"
/// ```
///
/// It also works on **tuple** structs, which is useful when implementing the
/// [New Type idiom](https://doc.rust-lang.org/rust-by-example/generics/new_types.html):
///
/// ```
/// use ddd_rs::domain::ValueObject;
///
/// #[derive(ddd_rs::ValueObject, Debug)]
/// struct MyTupleValueObject(#[eq_component] bool, Option<i32>, #[eq_component] String);
///
/// let a = MyTupleValueObject(true, Some(42), String::from("foo"));
/// let b = MyTupleValueObject(true, Some(-1), String::from("foo"));
/// let c = MyTupleValueObject(false, Some(42), String::from("bar"));
///
/// assert_eq!(a, b);
/// assert_ne!(a, c);
///
/// assert!(a > c);
/// ```
///
/// And, although kind of useless, on **unit** structs:
///
/// ```
/// use ddd_rs::domain::ValueObject;
///
/// #[derive(ddd_rs::ValueObject, Debug)]
/// struct MyUnitValueObject;
/// ```
pub trait ValueObject: Clone + PartialEq + PartialOrd {}
