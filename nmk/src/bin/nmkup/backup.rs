use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io};

use dirs::home_dir;

use nmk::home::NmkHome;

const TAG: &str = "backup";

fn time_since_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn should_backup_dir(dir_path: &Path) -> bool {
    let all_files = fs::read_dir(dir_path)
        .unwrap_or_else(|e| panic!("Failed to read dir {:?}. Error\n{:?}", dir_path, e))
        // We ignore unreadable files
        .flatten();
    let normal_files_count = all_files.filter(|p| p.file_name() != ".empty").count();
    normal_files_count > 0
}

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

pub fn backup_files(nmk_home: &NmkHome) -> io::Result<()> {
    let home = home_dir().expect("Failed to find home directory");
    let ar_path = home.join(format!("nmk-backup-{}.tar", time_since_epoch()));
    let ar = File::create(&ar_path)?;
    let mut ar = tar::Builder::new(BufWriter::new(ar));
    ar.follow_symlinks(false);
    for dir_name in BACKUP_DIRS {
        let dir_path = nmk_home.join(dir_name);
        if dir_path.exists() && should_backup_dir(&dir_path) {
            ar.append_dir_all(dir_name, &dir_path)?;
            log::debug!("{}: Added dir: {}", TAG, dir_name);
        }
    }
    for file_name in BACKUP_FILES {
        let file_path = nmk_home.join(file_name);
        if file_path.exists() {
            ar.append_path_with_name(&file_path, file_name)?;
            log::debug!("{}: Added file: {}", TAG, file_name);
        }
    }
    ar.finish()?;
    log::info!("{}: Important files are backup to {:?}", TAG, ar_path);
    Ok(())
}
