pub mod artifact;
pub mod bin_name;
pub mod container;
pub mod env_name;
pub mod home;
pub mod platform;
pub mod time;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
