use std::fs::OpenOptions;
use std::io;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use bytes::{Buf, Bytes};

use nmk::artifact::{download_file, download_file_metadata, Metadata};
use nmk::home::NmkHome;

use crate::build::Target;
use crate::cmdline::Opt;
use crate::ARTIFACT_BASE_URL;

fn unxz_entrypoint(data: Bytes, dst: impl AsRef<Path>) -> io::Result<u64> {
    let mut data_stream = xz2::read::XzDecoder::new(data.bytes());
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o755)
        .open(dst.as_ref())?;
    std::io::copy(&mut data_stream, &mut file)
}

const NMK_METADATA: &str = ".nmk.metadata";

pub async fn install_or_update(opt: &Opt, nmk_home: &NmkHome) -> nmk::Result<bool> {
    let nmk_home = nmk_home.as_ref();

    let target = Target::try_parse_env().unwrap();
    let tar_file = match target {
        Target::Amd64Linux => "nmk-amd64-linux-musl.xz",
        Target::Arm64Linux => "nmk-arm64-linux-musl.xz",
        Target::ArmV7Linux => "nmk-armv7-linux.xz",
    };
    let url = format!("{}/{}", ARTIFACT_BASE_URL, tar_file);

    let metadata_path = nmk_home.join(NMK_METADATA);

    let client = reqwest::Client::new();

    log::debug!("entrypoint: Getting metadata.");
    let metadata = download_file_metadata(&client, &url).await?;
    log::debug!("entrypoint: Received metadata.");
    log::debug!("entrypoint: etag {}", metadata.etag());
    if !opt.force && is_entrypoint_up2date(&metadata_path, &metadata) {
        log::info!("entrypoint: Already up to date.");
        Ok(false)
    } else {
        log::debug!("entrypoint: Getting data.");
        let data = download_file(&client, url).await?;
        log::debug!("entrypoint: Received data.");
        unxz_entrypoint(data, nmk_home.join("bin").join("nmk"))?;
        metadata
            .write_to_file(metadata_path)
            .expect("Unable to cache entrypoint metadata");
        log::info!("entrypoint: Done.");
        Ok(true)
    }
}

fn is_entrypoint_up2date(src: impl AsRef<Path>, metadata: &Metadata) -> bool {
    let src = src.as_ref();
    if !src.exists() {
        log::debug!("entrypoint: Not found cached metadata.");
        return false;
    }

    let cache_metadata =
        Metadata::read_from_file(src).expect("Fail to read or parse cached entrypoint metadata");
    log::debug!("entrypoint: cached etag {}", cache_metadata.etag());
    cache_metadata.etag() == metadata.etag()
}
