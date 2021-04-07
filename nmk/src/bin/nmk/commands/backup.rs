use std::path::PathBuf;

use nmk::backup::backup_files;
use nmk::home::NmkHome;

pub fn backup() -> nmk::Result<()> {
    let output_path = PathBuf::from("nmk-backup.tar");
    let nmk_home = NmkHome::locate().expect("failed to locate NMK_HOME");
    backup_files(&nmk_home, &output_path)?;
    Ok(())
}
