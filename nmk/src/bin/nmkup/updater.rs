use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

use bytes::{Buf, Bytes};

use nmk::artifact::download_file;
use nmk::home::NmkHome;

use crate::build::Target;
use crate::ARTIFACT_BASE_URL;

fn is_same_location(a: &PathBuf, b: &PathBuf) -> bool {
    fs::canonicalize(a).unwrap() == fs::canonicalize(b).unwrap()
}

pub async fn self_setup(
    nmk_home: &NmkHome,
    is_init: bool,
    entrypoint_updated: bool,
) -> nmk::Result<()> {
    let current_exec = env::current_exe().expect("current_exe() failed");
    let target_bin = nmk_home.join("bin").join("nmkup");
    let is_self_update =
        !is_init && target_bin.exists() && is_same_location(&current_exec, &target_bin);
    if is_self_update {
        if entrypoint_updated {
            perform_self_update_from_remote(target_bin).await?;
            log::info!("updater: Done.");
        }
    } else {
        fs::copy(current_exec, target_bin).expect("install nmkup failed");
        log::info!("updater: Done.");
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
    let url = format!("{}/{}", ARTIFACT_BASE_URL, tar_file);
    let client = reqwest::Client::new();
    let data = download_file(&client, url).await?;

    let target_bin = std::fs::canonicalize(target_bin)?;
    let parent_dir = target_bin
        .parent()
        .expect("updater: Unable to find parent directory.");
    let temp_target = parent_dir.join("nmkup.next");
    unxz_nmkup(data, &temp_target).expect("updater: Unable to extract data");
    std::fs::rename(temp_target, target_bin)?;
    Ok(())
}

fn unxz_nmkup(data: Bytes, dst: impl AsRef<Path>) -> io::Result<u64> {
    let mut xz = xz2::read::XzDecoder::new(data.bytes());
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o755)
        .open(dst.as_ref())?;
    std::io::copy(&mut xz, &mut file)
}
