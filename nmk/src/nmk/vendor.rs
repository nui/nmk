use std::io::BufRead;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::{fs, io};

use log::{debug, info};
use tar::Archive;
use xz2::bufread::XzDecoder;

const TAG: &str = "vendor";

pub fn extract_vendor_files(data: impl BufRead, destination: impl AsRef<Path>) -> io::Result<()> {
    let mut archive = Archive::new(XzDecoder::new(data));
    info!("{}: Installing to {}.", TAG, destination.as_ref().display());
    archive.unpack(destination)
}

pub fn prepare_vendor_dir(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref();
    if path.exists() {
        debug!("{}: Removing {} content.", TAG, path.display());
        remove_dir_contents(path)?;
    } else {
        fs::create_dir(path)?;
    }
    Ok(())
}

fn remove_dir_contents(path: impl AsRef<Path>) -> io::Result<()> {
    fs::read_dir(path)?.try_for_each(|entry| {
        let p = entry?.path();

        if p.is_dir() {
            fs::remove_dir_all(p)
        } else {
            // ignore removing dot files
            let file_name = p.file_name().expect("valid file name");
            if file_name.as_bytes().starts_with(b".") {
                return Ok(());
            }
            fs::remove_file(p)
        }
    })
}
