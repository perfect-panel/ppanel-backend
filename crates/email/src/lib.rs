pub mod manager;
pub mod platform;
pub mod sender;
pub mod smtp;
pub mod template;
pub mod worker;

pub use manager::{get_global_manager, set_global_manager, WorkerManager};
pub use platform::{get_supported_platforms, Platform, PlatformInfo};
pub use sender::{new_sender, EmailError, Sender};
pub use worker::{ErrorInfo, Worker, WorkerStatus};
