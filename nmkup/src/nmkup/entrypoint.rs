use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use flate2::read::GzDecoder;
use hyper::Uri;

use crate::nmkup::build::Target;
use crate::nmkup::client::SecureClient;
use crate::nmkup::BoxError;

fn unzip_entrypoint(file: File, dst: impl AsRef<Path>) {
    let dst = dst.as_ref();
    let mut gz = GzDecoder::new(file);
    let mut buf = Vec::new();
    gz.read_to_end(&mut buf)
        .expect("Unable to read encoded entrypoint");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o700)
        .open(dst)
        .unwrap();
    file.write_all(&buf).expect("Unable to write entrypoint");
}

pub async fn install(nmk_dir: impl AsRef<Path>) -> Result<(), BoxError> {
    let nmk_dir = nmk_dir.as_ref();
    let target = Target::try_parse_env().unwrap();
    let uri: Uri = match target {
        Target::Amd64Linux => {
            "https://storage.googleapis.com/nmk.nuimk.com/nmk.rs/nmk-amd64-linux-musl.gz"
        }
        Target::ArmV7Linux => {
            "https://storage.googleapis.com/nmk.nuimk.com/nmk.rs/nmk-armv7-linux.gz"
        }
    }
    .parse()?;
    let client = SecureClient::new();
    log::info!("Downloading entrypoint");
    let tar_gz = client.download_as_file(uri).await?;
    unzip_entrypoint(tar_gz, nmk_dir.join("bin").join("nmk"));
    log::info!("Extracted entrypoint");
    Ok(())
}
