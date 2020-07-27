pub mod bin_name;
pub mod container;
#[macro_use]
pub mod core;
pub mod env_name;
pub mod gcs;
pub mod home;
pub mod platform;
pub mod time;
pub mod tmux;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub const NMK_INIT_SCRIPT: &'static str = include_str!("../../nmkup.nuimk.com/nmkup-init.sh");
