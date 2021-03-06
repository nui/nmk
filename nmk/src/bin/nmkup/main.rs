use std::path::Path;
use std::process::exit;

use dirs::home_dir;
use log::error;

use nmk::backup::backup_files;
use nmk::home::NmkHome;
use nmk::platform;

mod build;
mod cmdline;
mod dotfiles;
mod entrypoint;
mod logging;
mod os_release;
mod updater;
mod vendor;

fn main() -> nmk::Result<()> {
    let cmd_opt = cmdline::from_args();
    logging::setup(cmd_opt.verbosity);
    // Installation should be done in order
    let nmk_home = NmkHome::find_for_install().expect("failed to locate NMK_HOME");
    if let Some(ref destination) = cmd_opt.download_and_install_entrypoint_to {
        entrypoint::download_and_install_to_file(destination)?;
        return Ok(());
    }
    if nmk_home.is_git() {
        error!("nmk is managed by git.");
        exit(1);
    }
    if platform::is_mac() {
        error!("Update on mac os is not supported");
        exit(1);
    }
    if cmd_opt.backup {
        let home = home_dir().expect("failed to find home directory");
        let output_tar = home.join("nmk-backup.tar");
        backup_files(&nmk_home, &output_tar)?;
    }
    dotfiles::install_or_update(&cmd_opt, &nmk_home)?;
    let entrypoint_installation = entrypoint::install_or_update(&cmd_opt, &nmk_home)?;
    updater::self_setup(&nmk_home, is_init(), entrypoint_installation)?;
    if cmd_opt.vendor {
        vendor::install(&cmd_opt, &nmk_home)?;
    }
    Ok(())
}

/// Check if this script is run from init script
///
/// We copy this behavior from rustup init script
fn is_init() -> bool {
    use std::os::unix::ffi::OsStrExt;
    std::env::current_exe()
        .ok()
        .as_deref()
        .and_then(Path::file_name)
        .expect("failed to find current executable file name")
        .as_bytes()
        .starts_with(b"nmkup-init")
}
