//! # ddd-rs-derive
//!
//! `ddd-rs`'s proc macros.

#![warn(missing_docs)]

mod aggregate_root;
mod entity;
mod value_object;

use proc_macro::TokenStream;

/// Proc macro for deriving the `AggregateRoot` trait.
///
/// Use the `#[aggregate_root(domain_events)]` attribute to tag the domain events field of the
/// aggregate root, which is assumed to be a `Vec`.
#[proc_macro_derive(AggregateRoot, attributes(aggregate_root))]
pub fn derive_aggregate_root(input: TokenStream) -> TokenStream {
    aggregate_root::derive(input)
}

/// Proc macro for deriving the `Entity` trait.
///
/// Use the `#[entity(id)]` attribute to tag the identity (ID) field of the entity.
#[proc_macro_derive(Entity, attributes(entity))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    entity::derive(input)
}

/// Proc macro for deriving the `ValueObject` trait.
///
/// Use the `#[value_object(eq)]` attribute to tag which fields should be considered as equality
/// components when comparing value objects.
#[proc_macro_derive(ValueObject, attributes(value_object))]
pub fn derive_value_object(input: TokenStream) -> TokenStream {
    value_object::derive(input)
}
