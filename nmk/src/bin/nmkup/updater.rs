use std::io::Read;
use std::path::Path;
use std::{env, fs, io};

use log::{debug, info};
use same_file::is_same_file;

use nmk::gcs::{download_file, get_object_meta, get_object_meta_url};
use nmk::home::NmkHome;
use nmk::setup::install;

use crate::build::Target;
use crate::entrypoint::EntrypointInstallation;

const TAG: &str = "updater";

pub fn self_setup(
    nmk_home: &NmkHome,
    is_init: bool,
    entrypoint_installation: EntrypointInstallation,
) -> nmk::Result<()> {
    let current_exec = env::current_exe()?;
    let target_bin = nmk_home.path().bin().join("nmkup");
    let is_self_update =
        !is_init && target_bin.exists() && is_same_file(&current_exec, &target_bin)?;
    if is_self_update {
        // Entrypoint and updater are built at the same time.
        // So we update updater if entrypoint is updated.
        if matches!(entrypoint_installation, EntrypointInstallation::Installed) {
            perform_self_update_from_remote(&target_bin)?;
            info!("{}: Done.", TAG);
        }
    } else {
        fs::copy(current_exec, target_bin)?;
        info!("{}: Done.", TAG);
    }
    Ok(())
}

pub fn perform_self_update_from_remote(target_bin: &Path) -> nmk::Result<()> {
    let target = Target::detect().expect("unsupported arch");
    let tar_file = target.remote_binary_name("nmkup");
    let meta_url = get_object_meta_url(&tar_file);
    debug!("{}: Getting metadata.", TAG);
    let meta = get_object_meta(&meta_url)?;
    debug!("{}: Received metadata.", TAG);
    let data_url = &meta.media_link;
    debug!("{}: Getting data from {}.", TAG, data_url);
    let data = download_file(data_url)?;

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

fn install_updater(data: impl Read, dst: &Path) -> io::Result<()> {
    let mut reader = xz2::read::XzDecoder::new(data);
    install(&mut reader, dst)
}
