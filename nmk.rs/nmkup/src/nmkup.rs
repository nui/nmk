use std::{env, fs};
use std::path::PathBuf;

fn is_same_location(a: &PathBuf, b: &PathBuf) -> bool {
    fs::canonicalize(a).unwrap() == fs::canonicalize(b).unwrap()
}

fn should_install(source: &PathBuf, target: &PathBuf) -> bool {
    !target.exists() || !is_same_location(source, target)
}

pub fn self_setup(nmk_dir: &PathBuf) {
    let current_exec = env::current_exe().expect("current_exe() failed");
    let target_bin = nmk_dir.join("bin").join("nmkup");
    if should_install(&current_exec, &target_bin) {
        fs::copy(current_exec, target_bin).expect("install nmkup failed");
        info!("Installed nmkup");
    }
}

pub fn find_nmkdir() -> PathBuf {
    const NMK_DIR: &str = "NMK_DIR";
    match std::env::var_os(NMK_DIR) {
        Some(nmk_dir) => nmk_dir.into(),
        None => {
            let nmk_dir = dirs::home_dir().expect("Can't find home directory").join(".nmk");
            info!("Using default {}: {:?}", NMK_DIR, &nmk_dir);
            nmk_dir
        }
    }
}