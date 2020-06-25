use std::path::PathBuf;

pub mod artifact;
pub mod env;
pub mod home;
pub mod time;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn nmk_home() -> std::result::Result<PathBuf, String> {
    let path = match std::env::var_os(env::NMK_HOME) {
        Some(s) => PathBuf::from(s),
        None => dirs::home_dir()
            .ok_or("Can't find home directory".to_string())?
            .join(".nmk"),
    };
    if path.exists() {
        Ok(path)
    } else {
        Err(format!("{:?} doesn't exist", path))
    }
}
