/// Trait for representing a **Notification**.
///
/// Notifications, unlike [Request](super::Request)s are events used to notify interested parties
/// of an occurrence, which might be handled by them.
///
/// # Examples
///
/// Derive its implementation using the [ddd_rs::Notification](crate::Notification) macro:
///
/// ```
/// #[derive(ddd_rs::Notification, Debug)]
/// enum MyNotification {
///     A(ANotification),
///     B(BNotification),
///     C(CNotification),
///     D(DNotification),
/// }
///
/// #[derive(ddd_rs::Notification, Debug)]
/// struct ANotification;
///
/// #[derive(ddd_rs::Notification, Debug)]
/// struct BNotification(bool);
///
/// #[derive(ddd_rs::Notification, Debug)]
/// struct CNotification(i32, u32);
///
/// #[derive(ddd_rs::Notification, Debug)]
/// struct DNotification {
///     foo: Option<String>,
///     bar: Vec<i32>,
/// }
///
/// let a: MyNotification = ANotification.into();
/// let b: MyNotification = BNotification(true).into();
/// let c: MyNotification = CNotification(-1, 1).into();
/// let d: MyNotification = DNotification{
///     foo: Some(String::from("foo")),
///     bar: vec![1, 2, 3],
/// }.into();
/// ```
pub trait Notification: Send {}
