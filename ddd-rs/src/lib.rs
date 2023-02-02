//! # ddd-rs
//!
//! Domain-Driven Design (DDD) building blocks, for Rust applications.
//!
//! > Most of the definitions on these docs are taken from Eric Evan's
//! > [Domain-Driven Design: Tackling Complexity in the Heart of Software](https://www.oreilly.com/library/view/domain-driven-design-tackling/0321125215/).
//!
//! ## Application Layer
//!
//! - [DomainEventHandler](application::DomainEventHandler)
//! - [Repository](application::Repository)
//! - [RequestHandler](application::RequestHandler)
//!
//! ## Domain Layer
//!
//! - [AggregateRoot](domain::AggregateRoot)
//! - [DomainEvent](domain::DomainEvent)
//! - [Entity](domain::Entity)
//! - [ValueObject](domain::ValueObject)
//!
//! ## Infrastructure Layer
//!
//! - Persistence
//!   - [DomainRepository](infrastructure::DomainRepository)
//!   - [InMemoryRepository](infrastructure::InMemoryRepository)
//!
//! ## Presentation Layer
//!
//! - [Request](presentation::Request)
//! - [Result](presentation::Result)

#![warn(missing_docs)]

/// **Application Layer**: Repository, Request (Command / Query) / Domain Event handlers, Providers'
/// interfaces
pub mod application;

/// **Domain Layer**: AggregateRoot, Entity, Value Object, Domain Event
pub mod domain;

/// **Infrastructure Layer**: Persistence, Providers' implementations
pub mod infrastructure;

/// **Presentation (Interface) Layer**: Request (Command / Query), DTOs
pub mod presentation;

#[cfg(feature = "derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate ddd_rs_derive;

#[cfg(feature = "derive")]
pub use ddd_rs_derive::*;
