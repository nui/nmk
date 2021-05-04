use std::path::Path;
use std::{env, fs, io};

use bytes::{Buf, Bytes};

use nmk::gcs::{download_file, get_object_meta, get_object_meta_url};
use nmk::home::NmkHome;
use nmk::setup::install;

use crate::build::Target;

const TAG: &str = "updater";

fn is_same_location(a: &Path, b: &Path) -> bool {
    match (fs::canonicalize(a), fs::canonicalize(b)) {
        (Ok(x), Ok(y)) => x == y,
        _ => false,
    }
}

pub async fn self_setup(
    nmk_home: &NmkHome,
    is_init: bool,
    entrypoint_updated: bool,
) -> nmk::Result<()> {
    let current_exec = env::current_exe()?;
    let target_bin = nmk_home.nmk_path().bin().join("nmkup");
    let is_self_update =
        !is_init && target_bin.exists() && is_same_location(&current_exec, &target_bin);
    if is_self_update {
        if entrypoint_updated {
            perform_self_update_from_remote(&target_bin).await?;
            log::info!("{}: Done.", TAG);
        }
    } else {
        fs::copy(current_exec, target_bin)?;
        log::info!("{}: Done.", TAG);
    }
    Ok(())
}

pub async fn perform_self_update_from_remote(target_bin: &Path) -> nmk::Result<()> {
    let target = Target::detect().expect("unsupported arch");
    let tar_file = target.remote_binary_name("nmkup");
    let client = reqwest::Client::new();
    let meta_url = get_object_meta_url(&tar_file);
    log::debug!("{}: Getting metadata.", TAG);
    let meta = get_object_meta(&client, &meta_url).await?;
    log::debug!("{}: Received metadata.", TAG);
    let data_url = &meta.media_link;
    log::debug!("{}: Getting data from {}.", TAG, data_url);
    let data = download_file(&client, data_url).await?;

    let target_bin = fs::canonicalize(target_bin)?;
    let parent_dir = target_bin
        .parent()
        .unwrap_or_else(|| panic!("{}: Failed to find parent directory.", TAG));
    let temp_target = parent_dir.join("nmkup.next");
    install_updater(data, &temp_target)
        .unwrap_or_else(|_| panic!("{}: Failed to extract data", TAG));
    fs::rename(temp_target, target_bin)?;
    Ok(())
}

fn install_updater(data: Bytes, dst: impl AsRef<Path>) -> io::Result<()> {
    let mut reader = xz2::read::XzDecoder::new(data.reader());
    install(&mut reader, dst)
}
