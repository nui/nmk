use std::convert::TryFrom;
use std::fs::{create_dir_all, File};
use std::path::{Path, PathBuf};
use std::process::Command;

use bytes::Bytes;
use flate2::read::GzDecoder;
use tar::Archive;

use crate::nmkup::arg::Opt;
use crate::nmkup::BoxError;
use crate::nmkup::client::SecureClient;
use crate::nmkup::gcloud::MetaData;

const META_FILE: &str = ".gcs.resource.json";

async fn unpack_nmktar<P: AsRef<Path>>(file: File, dst: P) -> Result<(), BoxError> {
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
    let dst = dst.as_ref();
    log::info!("Installing to {:?}", dst);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = dst.join(entry.path()?.strip_prefix(".nmk")?);
        entry.unpack(&path)?;
    }
    Ok(())
}

async fn download_metadata(client: &SecureClient) -> Result<Bytes, BoxError> {
    let uri = "https://www.googleapis.com/storage/v1/b/nmk.nuimk.com/o/nmk.tar.gz".parse()?;
    Ok(client.get_bytes(uri).await?)
}

fn is_up2date(dst: &Path, meta: &MetaData) -> bool {
    let cache_path = dst.join(META_FILE);
    if !cache_path.exists() {
        log::debug!("Not found GoogleCloudStorage API cache");
        false
    } else {
        let md5 = meta.md5();

        let raw_cached_meta = std::fs::read_to_string(&cache_path).expect("Fail read cached metadata");
        let cached_meta = raw_cached_meta.parse::<MetaData>().ok();
        let cached_md5 = cached_meta.as_ref().and_then(|x| x.md5());
        match (cached_md5, md5) {
            (Some(a), Some(b)) => a == b,
            _ => false
        }
    }
}

fn cache_metadata(dst: &Path, meta: &MetaData) {
    let cache_path = dst.join(META_FILE);
    std::fs::write(cache_path, meta.to_string()).expect("Fail to write metadata");
}

pub async fn install_or_update(opt: &Opt, nmk_dir: &PathBuf) -> Result<(), BoxError> {
    if !nmk_dir.exists() {
        create_dir_all(nmk_dir).expect("Can't create directory");
        log::info!("Created {:?} directory", nmk_dir);
    }

    // check if safe to install
    let nmk_dir_empty = nmk_dir.read_dir().unwrap().next().is_none();
    let meta_file_exist = nmk_dir.join(META_FILE).exists();
    if !opt.force && !nmk_dir_empty {
        assert!(meta_file_exist, "{:?} Missing cached metadata or directory is not empty", nmk_dir_empty);
    }

    let client = SecureClient::new();
    log::info!("Downloading archive");
    let meta = MetaData::try_from(&download_metadata(&client).await?).expect("Fail parse metadata");
    if !opt.force && is_up2date(nmk_dir, &meta) {
        log::info!("Already up to dated!")
    } else {
        if meta_file_exist {
            // uninstall old version
            let _ = Command::new("sh")
                .arg("uninstall.sh")
                .current_dir(nmk_dir)
                .status()
                .expect("fail to run sh");
        }

        let uri = "https://storage.googleapis.com/nmk.nuimk.com/nmk.tar.gz".parse()?;
        let tar_gz = client.download_as_file(uri).await?;
        unpack_nmktar(tar_gz, nmk_dir).await?;
        cache_metadata(nmk_dir, &meta);
        log::info!("Installed a new version")
    }
    Ok(())
}
