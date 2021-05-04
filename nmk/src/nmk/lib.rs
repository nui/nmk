#[macro_use]
#[path = "macros.rs"]
mod _macros;
pub mod arch;
pub mod backup;
pub mod bin_name;
pub mod config;
pub mod container;
pub mod env_name;
pub mod error;
pub mod gcs;
pub mod home;
pub mod human_time;
pub mod platform;
pub mod setup;
pub mod tmux;

pub type Result<T> = std::result::Result<T, error::Error>;
