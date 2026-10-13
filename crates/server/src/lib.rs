//! Headless network adapter; transformations run in supervised child processes.
pub mod config;
pub mod http;
pub mod jobs;
pub mod service;
pub mod worker;

pub use config::{Cli, Scope, ServerConfig};
pub use http::{router, serve};
