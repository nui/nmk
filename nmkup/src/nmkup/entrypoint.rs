use std::fs::OpenOptions;
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use flate2::read::GzDecoder;

use crate::nmkup::build::Target;
use crate::nmkup::BoxError;
use bytes::{Buf, Bytes};

fn gunzip_entrypoint(gziped_data: Bytes, dst: impl AsRef<Path>) {
    let mut gz = GzDecoder::new(gziped_data.bytes());
    let mut buf = Vec::new();
    gz.read_to_end(&mut buf)
        .expect("Unable to read encoded entrypoint");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o700)
        .open(dst.as_ref())
        .unwrap();
    file.write_all(&buf).expect("Unable to write entrypoint");
}

pub async fn install(nmk_dir: impl AsRef<Path>) -> Result<(), BoxError> {
    let nmk_dir = nmk_dir.as_ref();
    let target = Target::try_parse_env().unwrap();
    let tar_file = match target {
        Target::Amd64Linux => "nmk-amd64-linux-musl.gz",
        Target::ArmV7Linux => "nmk-armv7-linux.gz",
    };
    let base_uri = "https://storage.googleapis.com/nmk.nuimk.com/nmk.rs";
    let uri = format!("{}/{}", base_uri, tar_file);
    log::info!("Downloading entrypoint");
    let gzipped_data = reqwest::get(&uri).await?.bytes().await?;
    gunzip_entrypoint(gzipped_data, nmk_dir.join("bin").join("nmk"));
    log::info!("Extracted entrypoint");
    Ok(())
}
