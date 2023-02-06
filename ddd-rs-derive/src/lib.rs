//! # ddd-rs-derive
//!
//! `ddd-rs`'s proc macros.

#![warn(missing_docs)]

mod aggregate_root;
mod entity;
mod notification;
mod value_object;

use proc_macro::TokenStream;

/// Proc macro for deriving the `AggregateRoot` trait.
#[proc_macro_derive(AggregateRoot)]
pub fn derive_aggregate_root(input: TokenStream) -> TokenStream {
    aggregate_root::derive(input)
}

/// Proc macro for deriving the `Entity` trait.
#[proc_macro_derive(Entity, attributes(entity))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    entity::derive(input)
}

/// Proc macro for deriving the `Notification` trait.
#[proc_macro_derive(Notification, attributes(notification))]
pub fn derive_notification(input: TokenStream) -> TokenStream {
    notification::derive(input)
}

/// Proc macro for deriving the `ValueObject` trait.
///
/// Use the `#[eq_component]` attribute to tag which fields should be considered when comparing
/// value objects.
#[proc_macro_derive(ValueObject, attributes(eq_component))]
pub fn derive_value_object(input: TokenStream) -> TokenStream {
    value_object::derive(input)
}
