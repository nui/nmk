use std::env;
use std::fs::File;
use std::path::PathBuf;

use crate::pathenv::PathVec;

pub fn add_local_library(nmk_dir: &PathBuf) {
    const LD: &str = "LD_LIBRARY_PATH";

    let local_lib_dir = nmk_dir.join("local").join("lib");
    if local_lib_dir.exists() {
        let mut ps = match env::var_os(LD) {
            Some(path) => {
                debug!("{}: {:?}", LD, path);
                PathVec::parse(path)
            }
            None => PathVec::new(),
        };
        ps.push_front(local_lib_dir);
        let next_ld = ps.make();
        debug!("{}: {:?}", LD, &next_ld);
        env::set_var(LD, next_ld);
    }
}

pub fn dir() -> PathBuf {
    const NMK_DIR: &str = "NMK_DIR";
    let path = match env::var_os(NMK_DIR) {
        Some(s) => PathBuf::from(s),
        None => dirs::home_dir()
            .expect("Can't find home directory")
            .join(".nmk"),
    };
    if !path.exists() {
        panic!(format!("{:?} doesn't exist", path));
    }
    path
}

pub fn display_motd() {
    let mut stdout = std::io::stdout();
    vec!["/var/run/motd.dynamic", "/etc/motd"]
        .into_iter()
        .map(|p| PathBuf::from(p))
        .filter(|p| p.exists())
        .flat_map(|p| File::open(p))
        .for_each(|mut f| {
            let _ = std::io::copy(&mut f, &mut stdout);
        });
}