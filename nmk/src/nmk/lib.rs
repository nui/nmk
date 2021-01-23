#[macro_use]
#[path = "macros.rs"]
mod _macros;
pub mod arch;
pub mod bin_name;
pub mod config;
pub mod container;
pub mod env_name;
pub mod error;
pub mod gcs;
pub mod home;
pub mod platform;
pub mod time;
pub mod tmux;

pub type Result<T> = std::result::Result<T, error::Error>;

pub const NMK_INIT_SCRIPT: &str = include_str!("../../nmkup.nuimk.com/nmkup-init.sh");
