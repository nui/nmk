use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};
use std::path::{Path, PathBuf};

use log::{debug, info, warn};
use tar::Archive;
use xz2::bufread::XzDecoder;

use crate::home::NmkPath;

const TAG: &str = "dotfiles";

pub fn extract_dotfiles(data: impl BufRead, destination: &Path) -> crate::Result<()> {
    let mut archive = Archive::new(XzDecoder::new(data));
    info!("{}: Installing to {}", TAG, destination.display());
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        // Strip leading `.nmk`
        let path = strip_components(&path, 1);
        let target_path = destination.join(path);
        entry.unpack(target_path)?;
    }
    Ok(())
}

fn strip_components(path: &Path, n: usize) -> &Path {
    let mut components = path.components();
    components.by_ref().take(n).for_each(drop);
    components.as_path()
}

pub fn uninstall(nmk_path: &NmkPath) -> crate::Result<()> {
    let installed_files_list = nmk_path.dotfiles_file_list();
    // It is easier to read the whole file as string.
    // But we use this low level algorithm to show how rust can handle it with smallest
    // number of allocations.
    let mut file_list = BufReader::new(File::open(installed_files_list)?);
    // these buffers are reused so we don't allocate every loop iteration
    let mut line_buf = Vec::new();
    let mut path_buf = PathBuf::new();
    loop {
        line_buf.clear();
        // each file in file_list is separated by `\0`
        if file_list.read_until(b'\0', &mut line_buf)? == 0 {
            break;
        }
        let file_path = std::str::from_utf8(&line_buf)?
            .trim_end_matches('\0')
            .trim_start_matches("./");
        if file_path.is_empty() {
            // if we not check this condition, below code will try to remove nmk_path directory
            // which is not we want.
            continue;
        }
        path_buf = set_base_path(path_buf, nmk_path);
        path_buf.push(file_path);
        match fs::remove_file(&path_buf) {
            Ok(_) => {
                debug!("Removed {}", path_buf.display());
            }
            Err(err) if err.kind() == ErrorKind::NotFound => {
                warn!("Not found: {}", path_buf.display())
            }
            err => err?,
        }
    }
    Ok(())
}

fn set_base_path(path_buf: PathBuf, dst: &NmkPath) -> PathBuf {
    let mut s = path_buf.into_os_string();
    s.clear();
    s.push(dst);
    s.into()
}
