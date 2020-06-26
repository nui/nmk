use std::fs::create_dir_all;
use std::path::Path;
use std::process::Command;

use bytes::{Buf, Bytes};
use tar::Archive;
use xz2::read::XzDecoder;

use nmk::artifact::{download_file, download_file_metadata, Metadata};
use nmk::home::NmkHome;

use crate::cmdline::Opt;
use crate::ARTIFACT_BASE_URL;

const DOTFILES_METADATA: &str = ".dotfiles.metadata";

async fn untar_dotfiles<P: AsRef<Path>>(data: Bytes, dst: P) -> nmk::Result<()> {
    let dst = dst.as_ref();
    let mut archive = Archive::new(XzDecoder::new(data.bytes()));
    log::info!("dotfiles: Installing to {:?}.", dst);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = dst.join(entry.path()?.strip_prefix(".nmk")?);
        entry.unpack(&path)?;
    }
    Ok(())
}

fn is_dotfiles_up2date(src: impl AsRef<Path>, metadata: &Metadata) -> bool {
    let src = src.as_ref();
    if !src.exists() {
        log::debug!("dotfiles: Not found cached metadata.");
        return false;
    }
    let cached_metadata =
        Metadata::read_from_file(src).expect("dotfiles: Fail to read or parse cached metadata");
    log::debug!("dotfiles: cached etag {}", cached_metadata.etag());
    cached_metadata.etag() == metadata.etag()
}

pub async fn install_or_update(opt: &Opt, nmk_home: &NmkHome) -> nmk::Result<()> {
    if !nmk_home.exists() {
        create_dir_all(nmk_home).expect("Can't create NMK_HOME directory");
        log::info!("Created {:?} directory", nmk_home);
    }

    // check if it is safe to install
    let nmk_home_empty = nmk_home.read_dir()?.next().is_none();
    let metadata_do_exist = nmk_home.join(DOTFILES_METADATA).exists();
    if !opt.force && !nmk_home_empty {
        assert!(
            metadata_do_exist,
            "{:?} Missing cached metadata or directory is not empty",
            nmk_home_empty
        );
    }

    let dotfiles_url = format!("{}/{}", ARTIFACT_BASE_URL, "dotfiles.tar.xz");

    let client = reqwest::Client::new();
    log::info!("dotfiles: Getting metadata.");
    let metadata = download_file_metadata(&client, &dotfiles_url).await?;
    log::info!("dotfiles: Received metadata.");
    log::debug!("dotfiles: etag {}", metadata.etag());
    if !opt.force && is_dotfiles_up2date(nmk_home.join(DOTFILES_METADATA), &metadata) {
        log::info!("dotfiles: Already up to date.");
    } else {
        if metadata_do_exist {
            // uninstall old version
            let _ = Command::new("sh")
                .arg("uninstall.sh")
                .current_dir(nmk_home)
                .status()
                .expect("fail to run sh");
        }

        log::info!("dotfiles: Getting data.");
        let tar_xz_data = download_file(&client, &dotfiles_url).await?;
        log::info!("dotfiles: Received data.");
        untar_dotfiles(tar_xz_data, nmk_home).await?;
        metadata
            .write_to_file(nmk_home.join(DOTFILES_METADATA))
            .expect("Unable to cache dotfiles metadata");

        log::info!("dotfiles: Done.")
    }
    Ok(())
}
