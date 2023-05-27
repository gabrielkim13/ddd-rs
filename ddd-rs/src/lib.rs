//! # ddd-rs
//!
//! Domain-Driven Design (DDD) building blocks, for Rust applications.
//!
//! > Most of the definitions on these docs are taken from Eric Evan's
//! > [Domain-Driven Design: Tackling Complexity in the Heart of Software](https://www.oreilly.com/library/view/domain-driven-design-tackling/0321125215/).
//!
//! ## Application layer
//!
//! - [Repository](application::Repository)
//! - Service:
//!   - [Command](application::Command) / [Query](application::Query)
//!   - [Request](application::Request)
//!   - [RequestHandler](application::RequestHandler)
//!
//! ## Domain layer
//!
//! - [AggregateRoot](domain::AggregateRoot)
//! - [Entity](domain::Entity)
//! - [ValueObject](domain::ValueObject)
//!
//! ## Infrastructure layer
//!
//! - In-memory:
//!   - [InMemoryRepository](infrastructure::InMemoryRepository)

#![warn(missing_docs)]

/// Application layer
pub mod application;

/// Domain layer
pub mod domain;

/// Infrastructure layer
pub mod infrastructure;

mod error;
pub use error::*;

#[cfg(feature = "derive")]
pub use ddd_rs_derive::*;
