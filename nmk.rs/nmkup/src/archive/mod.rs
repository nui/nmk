use std::convert::TryFrom;
use std::fs::{create_dir_all, File};
use std::path::{Path, PathBuf};

use bytes::Bytes;
use flate2::read::GzDecoder;
use tar::Archive;

use crate::BoxError;
use crate::client::SecureClient;
use crate::gcloud::MetaData;

const META_FILE: &str = ".gcs.resource.json";

async fn unpack_nmktar<P: AsRef<Path>>(file: File, dst: P) -> Result<(), BoxError> {
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
    let dst = dst.as_ref();
    info!("Installing to {:?}", dst);
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
        debug!("Not found GoogleCloudStorage API cache");
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

fn check_and_prepare(dest: impl AsRef<Path>) {
    let dest = dest.as_ref();
    if !dest.exists() {
        create_dir_all(dest).expect("Can't create directory");
        info!("Created {:?} directory", dest);
    }

    // check if safe to install
    let meta_file = dest.join(META_FILE);
    let dest_is_empty = || dest.read_dir().unwrap().next().is_none();
    assert!(meta_file.exists() || dest_is_empty(), "{:?} Missing cached metadata or directory is not empty", dest);
}

pub async fn install_or_update() -> Result<(), BoxError> {
    const NMK_DIR: &str = "NMK_DIR";
    let nmk_dir: PathBuf = std::env::var_os("NMK_DIR").expect(&format!("missing {} environment", NMK_DIR)).into();
    check_and_prepare(&nmk_dir);

    let client = SecureClient::new();
    info!("Downloading archive");
    let meta = MetaData::try_from(&download_metadata(&client).await?).expect("Fail parse metadata");
    if is_up2date(&nmk_dir, &meta) {
        info!("Already up to dated!")
    } else {
        let uri = "https://storage.googleapis.com/nmk.nuimk.com/nmk.tar.gz".parse()?;
        let tar_gz = client.download_as_file(uri).await?;
        unpack_nmktar(tar_gz, &nmk_dir).await?;
        cache_metadata(&nmk_dir, &meta);
        info!("Installed a new version")
    }
    Ok(())
}
