use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::{fs, io};

use log::{debug, info};
use xz2::read::XzDecoder;

use nmk::home::{NmkHome, NmkPath};
use nmk::setup::install_busy;
use nmk::{dotfiles, vendor};

use crate::cmdline::Setup;

pub fn setup(options: Setup) -> nmk::Result<()> {
    let nmk_home = NmkHome::find_for_install().expect("failed to locate dotfiles directory");
    assert!(
        !nmk_home.is_git(),
        "nmk is managed by git. Abort installation"
    );
    let nmk_path = nmk_home.path();
    // dotfiles must be installed first to get /bin for entrypoint
    if let Some(source) = options.dotfiles {
        assert!(source.exists(), "Not found dot files source");
        setup_dotfiles(&source, nmk_path)?;
        // remove old metadata if exists
        let metadata = nmk_path.dotfiles_meta();
        if metadata.exists() {
            fs::remove_file(metadata)?;
        }
    }
    if let Some(source) = options.entrypoint {
        assert!(source.exists(), "Not found entrypoint source");
        setup_entrypoint(&source, nmk_path)?;
        // remove old metadata if exists
        let metadata = nmk_path.entrypoint_meta();
        if metadata.exists() {
            fs::remove_file(metadata)?;
        }
    }
    if let Some(source) = options.vendor {
        assert!(source.exists(), "Not found vendor source");
        extract_vendor(&source, nmk_path)?;
    }
    Ok(())
}

fn setup_dotfiles(source: &Path, nmk_path: &NmkPath) -> nmk::Result<()> {
    let dotfiles_dir = nmk_path.as_path();
    if !dotfiles_dir.exists() {
        fs::create_dir_all(dotfiles_dir)?;
    }
    if nmk_path.dotfiles_file_list().exists() {
        dotfiles::uninstall(nmk_path)?;
    }
    let data = BufReader::new(File::open(source)?);
    dotfiles::extract_dotfiles(data, nmk_path.as_path())
}

fn setup_entrypoint(source: &Path, nmk_path: &NmkPath) -> io::Result<()> {
    let mut source = XzDecoder::new(fs::File::open(source)?);
    install_busy(&mut source, &nmk_path.entrypoint())?;
    info!("Entrypoint installed");
    Ok(())
}

fn extract_vendor(source: &Path, nmk_path: &NmkPath) -> io::Result<()> {
    let vendor_dir = nmk_path.vendor();
    debug!("Preparing vendor directory");
    vendor::prepare_vendor_dir(&vendor_dir)?;
    debug!("Extracting vendor files");
    let data = BufReader::new(File::open(source)?);
    vendor::extract_vendor_files(data, &vendor_dir)?;
    info!("Vendor files extracted");
    Ok(())
}
