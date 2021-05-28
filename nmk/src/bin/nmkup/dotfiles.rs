use std::fs;
use std::path::Path;
use std::process::Command;

use bytes::Bytes;
use tar::Archive;
use xz2::read::XzDecoder;

use nmk::gcs::{download_file, get_object_meta, get_object_meta_url, ObjectMeta};
use nmk::home::NmkHome;

use crate::cmdline::CmdOpt;

const DOTFILES_META: &str = ".dotfiles.meta";
const TAG: &str = "dotfiles";

async fn extract_dotfiles<P: AsRef<Path>>(data: Bytes, destination: P) -> nmk::Result<()> {
    let destination = destination.as_ref();
    let mut archive = Archive::new(XzDecoder::new(&*data));
    log::info!("{}: Installing to {}.", TAG, destination.display());
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        // Strip .nmk
        let new_path = strip_components(&path, 1);
        let full_path = destination.join(new_path);
        entry.unpack(full_path)?;
    }
    Ok(())
}

fn strip_components(path: &Path, n: usize) -> &Path {
    let mut components = path.components();
    components.by_ref().take(n).for_each(drop);
    components.as_path()
}

fn is_dotfiles_up2date(meta_path: &Path, gcs_meta: &ObjectMeta) -> bool {
    if !meta_path.exists() {
        log::debug!("{}: Not found cached metadata.", TAG);
        return false;
    }
    let cached_meta = ObjectMeta::read_from_file(meta_path);
    log::debug!("{}: gcs generation {}.", TAG, gcs_meta.generation);
    log::debug!("{}: cached generation {}.", TAG, cached_meta.generation);
    cached_meta.generation == gcs_meta.generation
}

pub async fn install_or_update(cmd_opt: &CmdOpt, nmk_home: &NmkHome) -> nmk::Result<()> {
    if !nmk_home.as_path().exists() {
        fs::create_dir_all(nmk_home)?;
        log::info!("Created {} directory", nmk_home.as_path().display());
    }

    let meta_path = nmk_home.as_path().join(DOTFILES_META);

    // check if it is safe to install
    let nmk_home_empty = nmk_home.as_path().read_dir()?.next().is_none();
    let meta_do_exist = meta_path.exists();
    if !cmd_opt.force && !nmk_home_empty {
        assert!(
            meta_do_exist,
            "Missing dotfiles metadata or directory is not empty",
        );
    }

    let meta_url = get_object_meta_url("dotfiles.tar.xz");

    let client = reqwest::Client::new();
    log::debug!("{}: Getting metadata.", TAG);
    let meta = get_object_meta(&client, &meta_url).await?;
    log::debug!("{}: Received metadata.", TAG);
    if !cmd_opt.force && is_dotfiles_up2date(&meta_path, &meta) {
        log::info!("{}: Already up to date.", TAG);
    } else {
        if meta_do_exist {
            // uninstall old version
            let _ = Command::new("sh")
                .arg("uninstall.sh")
                .current_dir(nmk_home)
                .status()
                .expect("fail to run sh");
        }

        log::debug!("{}: Getting data.", TAG);
        let tar_xz_data = download_file(&client, &meta.media_link).await?;
        log::debug!("{}: Received data.", TAG);
        extract_dotfiles(tar_xz_data, nmk_home).await?;
        meta.write_to_file(&meta_path);
        log::info!("{}: Done.", TAG)
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
