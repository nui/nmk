use std::io;
use std::io::Read;
use std::path::Path;

use log::{debug, info};

use nmk::gcs::{download_file, get_object_meta, get_object_meta_url, ObjectMeta};
use nmk::home::NmkHome;
use nmk::setup::install;

use crate::build::Target;
use crate::cmdline::CmdOpt;

const TAG: &str = "entrypoint";

fn install_entrypoint(data: impl Read, dst: impl AsRef<Path>) -> io::Result<()> {
    let mut reader = xz2::read::XzDecoder::new(data);
    install(&mut reader, dst)
}

#[derive(Copy, Clone)]
pub enum EntrypointInstallation {
    Installed,
    Up2Date,
}

pub fn install_or_update(
    cmd_opt: &CmdOpt,
    nmk_home: &NmkHome,
) -> nmk::Result<EntrypointInstallation> {
    let target = Target::detect().expect("unsupported arch");
    let tar_file = target.remote_binary_name("nmk");
    let meta_path = nmk_home.path().entrypoint_meta();
    let meta_url = get_object_meta_url(&tar_file);

    debug!("{}: Getting metadata.", TAG);
    let meta = get_object_meta(&meta_url)?;
    debug!("{}: Received metadata.", TAG);
    let entrypoint_path = nmk_home.path().entrypoint();
    if !cmd_opt.force && is_entrypoint_up2date(&meta_path, &meta, &entrypoint_path) {
        info!("{}: Already up to date.", TAG);
        Ok(EntrypointInstallation::Up2Date)
    } else {
        let data_url = &meta.media_link;
        debug!("{}: Getting data from {}.", TAG, data_url);
        let data = download_file(data_url)?;
        debug!("{}: Received data.", TAG);
        install_entrypoint(data, entrypoint_path)?;
        meta.write_to_file(&meta_path);
        info!("{}: Done.", TAG);
        Ok(EntrypointInstallation::Installed)
    }
}

fn is_entrypoint_up2date(meta_path: &Path, gcs_meta: &ObjectMeta, entrypoint_path: &Path) -> bool {
    if !entrypoint_path.exists() {
        return false;
    }
    if !meta_path.exists() {
        debug!("{}: Not found cached metadata.", TAG);
        return false;
    }

    let cached_meta = ObjectMeta::read_from_file(meta_path);
    debug!("{}: gcs generation {}.", TAG, gcs_meta.generation);
    debug!("{}: cached generation {}.", TAG, cached_meta.generation);
    cached_meta.generation == gcs_meta.generation
}
