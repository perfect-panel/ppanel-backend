//! Database entity types.
//!
//! Entity types describe the persisted shape of domain data. They are
//! distinct from the API DTOs in [`super::dto`] so internal columns and
//! validation rules can evolve independently of the public schema.

pub mod ads;
pub mod announcement;
pub mod auth;
pub mod client;
pub mod coupon;
pub mod document;
pub mod log;
pub mod node;
pub mod order;
pub mod payment;
pub mod subscribe;
pub mod system;
pub mod task;
pub mod ticket;
pub mod traffic;
pub mod user;
