//! Domain models for the ppanel backend.
//!
//! Re-exports the database entities (see `entity/`) and the API DTOs
//! (see `dto.rs`) so callers can simply `use crate::model::*`.

pub mod dto;

pub mod entity;