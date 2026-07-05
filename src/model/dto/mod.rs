//! API DTO types, split by domain.
//! All types re-exported so `model::dto::SomeType` paths continue to work.

pub mod ads;
pub mod announcement;
pub mod application;
pub mod auth;
pub mod common;
pub mod coupon;
pub mod document;
pub mod log;
pub mod marketing;
pub mod node;
pub mod order;
pub mod payment;
pub mod protocol;
pub mod server;
pub mod subscribe;
pub mod system;
pub mod ticket;
pub mod user;
pub mod misc;

pub use ads::*;
pub use announcement::*;
pub use application::*;
pub use auth::*;
pub use common::*;
pub use coupon::*;
pub use document::*;
pub use log::*;
pub use marketing::*;
pub use node::*;
pub use order::*;
pub use payment::*;
pub use protocol::*;
pub use server::*;
pub use subscribe::*;
pub use system::*;
pub use ticket::*;
pub use user::*;
pub use misc::*;
