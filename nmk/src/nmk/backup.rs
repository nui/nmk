use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::{fs, io};

use crate::home::NmkHome;

#[rustfmt::skip]
const BACKUP_FILES: &[&str] = &[
    ".tmux_history",
    "zsh/.zsh_history",
    "zsh/zprofile",
    "zsh/zshenv.extra",
];

#[rustfmt::skip]
const BACKUP_DIRS: &[&str] = &[
    "zsh/completion",
    "zsh/zshrc.extra.d",
    "zsh/zshrc.pre.d",
];

fn should_backup_dir(dir: &Path) -> bool {
    fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("Failed to read dir {:?}. Error\n{:?}", dir, e))
        // We ignore unreadable files
        .flatten()
        .any(|p| p.file_name() != ".empty")
}

pub fn backup_files(nmk_home: &NmkHome, ar_path: &Path) -> io::Result<()> {
    let mut ar = tar::Builder::new(BufWriter::new(File::create(&ar_path)?));
    let base_dir = nmk_home.as_path();
    ar.follow_symlinks(false);
    for name in BACKUP_DIRS {
        let dir = base_dir.join(name);
        if dir.exists() && should_backup_dir(&dir) {
            ar.append_dir_all(name, &dir)?;
            log::debug!("Added dir: {}", name);
        }
    }
    for name in BACKUP_FILES {
        let file = base_dir.join(name);
        if let Ok(mut f) = File::open(file) {
            ar.append_file(name, &mut f)?;
            log::debug!("Added file: {}", name);
        }
    }
    ar.finish()?;
    log::info!("Important files are backup to {:?}", ar_path);
    Ok(())
}
