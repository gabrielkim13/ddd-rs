use crate::presentation::Notification;

/// Result type for [NotificationHandler] operations.
pub type Result<T, E = Box<dyn std::error::Error + Send + Sync>> = core::result::Result<T, E>;

/// Trait for representing a **Notification Handler**.
///
/// See [Notification] for more information about Notifications.
///
/// # Examples
///
/// Implement the [NotificationHandler] trait for each [Notification] variant (struct). Then, use
/// the [ddd_rs::Notification](crate::Notification) macro, and the `handler` attribute to derive the
/// trait implementation for the main [Notification] enum:
///
/// ```
/// use ddd_rs::application::notification_handler::{self, NotificationHandler};
/// use ddd_rs::presentation::Notification;
///
/// #[derive(ddd_rs::Notification, Debug)]
/// #[notification(handler = "MyNotificationHandler")]
/// enum MyNotification {
///     A(ANotification),
///     B(BNotification),
///     C(CNotification),
/// }
///
/// #[derive(ddd_rs::Notification, Debug)]
/// struct ANotification {
///     a_field: bool,
/// }
///
/// #[derive(ddd_rs::Notification, Debug)]
/// struct BNotification {
///     b_field: i32,
/// }
///
/// #[derive(ddd_rs::Notification, Debug)]
/// struct CNotification {
///     c_field: String,
/// }
///
/// struct MyNotificationHandler;
///
/// #[async_trait::async_trait]
/// impl NotificationHandler<ANotification> for MyNotificationHandler {
///     async fn handle(&self, notification: ANotification) -> notification_handler::Result<()> {
///         match notification.a_field {
///             true => Ok(()),
///             _ => Err("a_field is not true".into()),
///         }
///     }
/// }
///
/// #[async_trait::async_trait]
/// impl NotificationHandler<BNotification> for MyNotificationHandler {
///     async fn handle(&self, notification: BNotification) -> notification_handler::Result<()> {
///         match notification.b_field {
///             1 => Ok(()),
///             _ => Err("b_field is not 1".into()),
///         }
///     }
/// }
///
/// #[async_trait::async_trait]
/// impl NotificationHandler<CNotification> for MyNotificationHandler {
///     async fn handle(&self, notification: CNotification) -> notification_handler::Result<()> {
///         match notification.c_field.as_str() {
///             "1" => Ok(()),
///             _ => Err("c_field is not \"1\"".into()),
///         }
///     }
/// }
///
/// # tokio_test::block_on(async {
/// let a: MyNotification = ANotification { a_field: true }.into();
/// let b: MyNotification = BNotification { b_field: 42 }.into();
/// let c: MyNotification = CNotification { c_field: String::from("1") }.into();
///
/// assert!(MyNotificationHandler.handle(a).await.is_ok());
/// assert!(MyNotificationHandler.handle(b).await.is_err());
/// assert!(MyNotificationHandler.handle(c).await.is_ok());
/// # })
/// ```
#[async_trait::async_trait]
pub trait NotificationHandler<T: Notification>: Send + Sync {
    /// Handles the incoming [Notification] returning a unitary [Result].
    async fn handle(&self, notification: T) -> Result<()>;
}
