use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::{fs, io};

use log::{debug, info};

use crate::home::NmkHome;

const BACKUP_PATHS: &[&str] = &[
    ".tmux_history",
    "zsh/.zsh_history",
    "zsh/completion/",
    "zsh/zprofile",
    "zsh/zshenv.extra",
    "zsh/zshrc.extra.d/",
    "zsh/zshrc.pre.d/",
];

fn should_backup_dir(dir: &Path) -> bool {
    fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("Failed to read dir {}. Error\n{:?}", dir.display(), e))
        .flatten() // We ignore any IO error
        .any(|p| p.file_name() != ".empty")
}

pub fn backup_files(nmk_home: &NmkHome, ar_path: &Path) -> io::Result<()> {
    let mut ar = tar::Builder::new(BufWriter::new(File::create(&ar_path)?));
    ar.follow_symlinks(false);
    let mut dirs = vec![];
    let mut files = vec![];
    let nmk_home_path = nmk_home.path().as_path();
    for name in BACKUP_PATHS {
        let path = nmk_home_path.join(name);
        if path.is_dir() {
            if should_backup_dir(&path) {
                dirs.push((name, path));
            }
        } else if path.is_file() {
            if let Ok(f) = File::open(&path) {
                files.push((name, f))
            }
        }
    }
    for (name, d) in dirs {
        ar.append_dir_all(name, d)?;
        debug!("Added dir: {}", name);
    }
    for (name, mut f) in files {
        ar.append_file(name, &mut f)?;
        debug!("Added file: {}", name);
    }
    ar.finish()?;
    info!("Important files are backup to {}", ar_path.display());
    Ok(())
}
