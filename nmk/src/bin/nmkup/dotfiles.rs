use std::fs;
use std::path::Path;
use std::process::Command;

use bytes::Bytes;
use tar::Archive;

use nmk::gcs::{download_file, get_object_meta, get_object_meta_url, ObjectMeta};
use nmk::home::NmkHome;

use crate::cmdline::CmdOpt;

const DOTFILES_META: &str = ".dotfiles.meta";
const TAG: &str = "dotfiles";

async fn extract_dotfiles<P: AsRef<Path>>(data: Bytes, dst: P) -> nmk::Result<()> {
    let dst = dst.as_ref();
    let tar_data_stream = xz2::bufread::XzDecoder::new(&*data);
    let mut archive = Archive::new(tar_data_stream);
    log::info!("{}: Installing to {:?}.", TAG, dst);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        let mut entry_path = path.components();
        entry_path.next(); // Strip the first component (.nmk)
        let full_path = dst.join(entry_path);
        entry.unpack(full_path)?;
    }
    Ok(())
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
    if !nmk_home.exists() {
        fs::create_dir_all(nmk_home)?;
        log::info!("Created {:?} directory", nmk_home);
    }

    let meta_path = nmk_home.join(DOTFILES_META);

    // check if it is safe to install
    let nmk_home_empty = nmk_home.read_dir()?.next().is_none();
    let meta_do_exist = meta_path.exists();
    if !cmd_opt.force && !nmk_home_empty {
        assert!(
            meta_do_exist,
            "{:?} Missing dotfiles metadata or directory is not empty",
            nmk_home_empty
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
