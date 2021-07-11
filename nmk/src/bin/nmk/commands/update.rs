use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::{fs, io};

use log::{debug, info};
use xz2::read::XzDecoder;

use nmk::home::{NmkHome, NmkPath};
use nmk::setup::install_busy;
use nmk::{dotfiles, vendor};

use crate::cmdline::Update;

/// Update nmk from files
pub fn file_update(update: Update) -> nmk::Result<()> {
    let nmk_home = NmkHome::locate().expect("failed to locate dotfiles directory");
    assert!(!nmk_home.is_git(), "nmk is managed by git. Abort updating");
    if let Some(path) = update.entrypoint {
        assert!(path.exists(), "Not found entrypoint source");
        update_entrypoint(&path, nmk_home.path())?;
        let metadata = nmk_home.path().entrypoint_meta();
        if metadata.exists() {
            fs::remove_file(metadata)?;
        }
    }
    if let Some(path) = update.dotfiles {
        assert!(path.exists(), "Not found dot files source");
        dotfiles::uninstall(&nmk_home)?;
        let data = BufReader::new(File::open(&path)?);
        dotfiles::extract_dotfiles(data, nmk_home.path())?;
        let metadata = nmk_home.path().dotfiles_meta();
        if metadata.exists() {
            fs::remove_file(metadata)?;
        }
    }
    if let Some(path) = update.vendor {
        assert!(path.exists(), "Not found vendor source");
        let vendor_dir = nmk_home.path().vendor();
        let source = path.canonicalize()?;
        update_vendor(&source, &vendor_dir)?;
    }
    Ok(())
}

fn update_entrypoint(source: &Path, nmk_path: &NmkPath) -> io::Result<()> {
    let mut source = XzDecoder::new(fs::File::open(source)?);
    install_busy(&mut source, nmk_path.entrypoint())?;
    info!("Entrypoint updated");
    Ok(())
}

fn update_vendor(source: &Path, target_dir: &Path) -> io::Result<()> {
    debug!("Preparing vendor directory");
    vendor::prepare_vendor_dir(target_dir)?;
    debug!("Extracting vendor files");
    let data = BufReader::new(File::open(source)?);
    vendor::extract_vendor_files(data, target_dir)?;
    info!("Vendor files extracted");
    Ok(())
}
