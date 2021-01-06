use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

use bytes::Bytes;

use nmk::gcs::{download_file, get_object_meta, get_object_meta_url};
use nmk::home::NmkHome;

use crate::build::Target;

const TAG: &str = "updater";

fn is_same_location(a: &PathBuf, b: &PathBuf) -> bool {
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
            perform_self_update_from_remote(target_bin).await?;
            log::info!("{}: Done.", TAG);
        }
    } else {
        fs::copy(current_exec, target_bin)?;
        log::info!("{}: Done.", TAG);
    }
    Ok(())
}

pub async fn perform_self_update_from_remote(target_bin: PathBuf) -> nmk::Result<()> {
    let target = Target::try_parse_env().unwrap();
    let tar_file = match target {
        Target::Amd64Linux => "nmkup-x86_64-unknown-linux-musl.xz",
        Target::Arm64Linux => "nmkup-aarch64-unknown-linux-musl.xz",
        Target::ArmLinux | Target::ArmV7Linux => "nmkup-arm-unknown-linux-musleabi.xz",
    };
    let client = reqwest::Client::new();
    let meta_url = get_object_meta_url(tar_file);
    log::debug!("{}: Getting metadata.", TAG);
    let meta = get_object_meta(&client, &meta_url).await?;
    log::debug!("{}: Received metadata.", TAG);
    let data = download_file(&client, &meta.media_link).await?;

    let target_bin = fs::canonicalize(target_bin)?;
    let parent_dir = target_bin
        .parent()
        .unwrap_or_else(|| panic!("{}: Unable to find parent directory.", TAG));
    let temp_target = parent_dir.join("nmkup.next");
    unxz_nmkup(data, &temp_target).unwrap_or_else(|_| panic!("{}: Unable to extract data", TAG));
    fs::rename(temp_target, target_bin)?;
    Ok(())
}

fn unxz_nmkup(data: Bytes, dst: impl AsRef<Path>) -> io::Result<u64> {
    let mut xz = xz2::read::XzDecoder::new(&*data);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o755)
        .open(dst)?;
    io::copy(&mut xz, &mut file)
}
