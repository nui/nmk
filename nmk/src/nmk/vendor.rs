use std::io::BufRead;
use std::path::Path;
use std::{fs, io};

use log::info;
use tar::Archive;
use xz2::bufread::XzDecoder;

const TAG: &str = "vendor";

pub fn extract_vendor_files(data: impl BufRead, destination: &Path) -> io::Result<()> {
    let mut archive = Archive::new(XzDecoder::new(data));
    info!("{}: Installing to {}.", TAG, destination.display());
    archive.unpack(destination)
}

pub fn prepare_vendor_dir(path: &Path) -> io::Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    fs::create_dir(path)?;
    Ok(())
}
