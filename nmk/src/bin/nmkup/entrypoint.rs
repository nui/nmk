use std::fs::OpenOptions;
use std::io;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use bytes::{Buf, Bytes};

use nmk::bin_name::NMK;
use nmk::gcs::{download_file, get_object_meta, get_object_meta_url, ObjectMeta};
use nmk::home::NmkHome;

use crate::build::Target;
use crate::cmdline::Opt;

const TAG: &str = "entrypoint";

fn unxz_entrypoint(data: Bytes, dst: impl AsRef<Path>) -> io::Result<u64> {
    let mut data_stream = xz2::read::XzDecoder::new(data.bytes());
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o755)
        .open(dst)?;
    io::copy(&mut data_stream, &mut file)
}

const NMK_META: &str = ".nmk.meta";

pub async fn install_or_update(opt: &Opt, nmk_home: &NmkHome) -> nmk::Result<bool> {
    let nmk_home = nmk_home.as_ref();

    let target = Target::try_parse_env().unwrap();
    let tar_file = match target {
        Target::Amd64Linux => "nmk-x86_64-unknown-linux-musl.xz",
        Target::Arm64Linux => "nmk-aarch64-unknown-linux-musl.xz",
        Target::ArmLinux | Target::ArmV7Linux => "nmk-arm-unknown-linux-musleabi.xz",
    };
    let meta_path = nmk_home.join(NMK_META);
    let meta_url = get_object_meta_url(tar_file);

    let client = reqwest::Client::new();

    log::debug!("{}: Getting metadata.", TAG);
    let meta = get_object_meta(&client, &meta_url).await?;
    log::debug!("{}: Received metadata.", TAG);
    let entrypoint_path = nmk_home.join("bin").join(NMK);
    if !opt.force && is_entrypoint_up2date(&meta_path, &meta, &entrypoint_path) {
        log::info!("{}: Already up to date.", TAG);
        Ok(false)
    } else {
        log::debug!("{}: Getting data.", TAG);
        let data = download_file(&client, &meta.media_link).await?;
        log::debug!("{}: Received data.", TAG);
        unxz_entrypoint(data, entrypoint_path)?;
        meta.write_to_file(&meta_path);
        log::info!("{}: Done.", TAG);
        Ok(true)
    }
}

fn is_entrypoint_up2date(meta_path: &Path, gcs_meta: &ObjectMeta, entrypoint_path: &Path) -> bool {
    if !entrypoint_path.exists() {
        return false;
    }
    if !meta_path.exists() {
        log::debug!("{}: Not found cached metadata.", TAG);
        return false;
    }

    let cached_meta = ObjectMeta::read_from_file(meta_path);
    log::debug!("{}: gcs generation {}.", TAG, gcs_meta.generation);
    log::debug!("{}: cached generation {}.", TAG, cached_meta.generation);
    cached_meta.generation == gcs_meta.generation
}
