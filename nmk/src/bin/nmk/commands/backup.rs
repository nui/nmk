use nmk::home::NmkHome;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn backup() -> nmk::Result<()> {
    let output_path = PathBuf::from("nmk-backup.tar");
    let mut ar = tar::Builder::new(BufWriter::new(File::create(&output_path)?));
    ar.follow_symlinks(false);
    let nmk_home = NmkHome::locate().expect("Unable to locate NMK_HOME");
    let files = &[".tmux_history", "zsh/.zsh_history", "zsh/zprofile"];
    let dirs = &["zsh/zshrc.extra.d"];
    for file in files {
        let path = nmk_home.as_path().join(file);
        if let Ok(mut f) = File::open(path) {
            ar.append_file(file, &mut f)?;
            println!("Backup {}", file);
        }
    }
    for dir in dirs {
        let path = nmk_home.as_path().join(dir);
        ar.append_dir_all(dir, path)?;
        println!("Backup {}", dir);
    }
    ar.into_inner()?.flush()?;
    println!("Backup to {:?}", std::fs::canonicalize(&output_path)?);
    Ok(())
}
