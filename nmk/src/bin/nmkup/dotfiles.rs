use std::fs;
use std::io::BufReader;
use std::path::Path;

use log::{debug, info};

use nmk::dotfiles::{extract_dotfiles, uninstall};
use nmk::gcs::{download_file, get_object_meta, get_object_meta_url, ObjectMeta};
use nmk::home::NmkHome;

use crate::cmdline::CmdOpt;

const TAG: &str = "dotfiles";

fn is_dotfiles_up2date(meta_path: &Path, gcs_meta: &ObjectMeta) -> bool {
    if !meta_path.exists() {
        debug!("{}: Not found cached metadata.", TAG);
        return false;
    }
    let cached_meta = ObjectMeta::read_from_file(meta_path);
    debug!("{}: gcs generation {}.", TAG, gcs_meta.generation);
    debug!("{}: cached generation {}.", TAG, cached_meta.generation);
    cached_meta.generation == gcs_meta.generation
}

pub fn install_or_update(cmd_opt: &CmdOpt, nmk_home: &NmkHome) -> nmk::Result<()> {
    let nmk_home_path = nmk_home.path().as_path();
    if !nmk_home_path.exists() {
        fs::create_dir_all(nmk_home_path)?;
        info!("Created {} directory", nmk_home_path.display());
    }

    let meta_path = nmk_home.path().dotfiles_meta();

    // check if it is safe to install
    let nmk_home_empty = nmk_home_path.read_dir()?.next().is_none();
    let meta_do_exist = meta_path.exists();
    if !cmd_opt.force && !nmk_home_empty {
        assert!(
            meta_do_exist,
            "Missing dotfiles metadata or directory is not empty",
        );
    }

    let meta_url = get_object_meta_url("dotfiles.tar.xz");

    debug!("{}: Getting metadata.", TAG);
    let meta = get_object_meta(&meta_url)?;
    debug!("{}: Received metadata.", TAG);
    if !cmd_opt.force && is_dotfiles_up2date(&meta_path, &meta) {
        info!("{}: Already up to date.", TAG);
    } else {
        if meta_do_exist {
            // uninstall old version, we don't care if it success or not
            uninstall(nmk_home.path())?;
        }

        debug!("{}: Getting data.", TAG);
        let tar_xz_data = BufReader::new(download_file(&meta.media_link)?);
        debug!("{}: Received data.", TAG);
        extract_dotfiles(tar_xz_data, nmk_home_path)?;
        meta.write_to_file(&meta_path);
        info!("{}: Done.", TAG)
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_first_component() {
        let p = Path::new(".nmk/bin/nmk");
        let actual = strip_components(&p, 1);
        assert_eq!(actual, Path::new("bin/nmk"));
        let actual = strip_components(&p, 2);
        assert_eq!(actual, Path::new("nmk"));
    }
}
