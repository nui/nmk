use std::path::Path;

use dirs::home_dir;

use nmk::backup::backup_files;
use nmk::home::NmkHome;
use nmk::platform;

mod build;
mod cmdline;
mod config;
mod dotfiles;
mod entrypoint;
mod logging;
mod os_release;
mod updater;
mod vendor;

async fn main_task(cmd_opt: cmdline::CmdOpt, _settings: config::Config) -> nmk::Result<()> {
    // Installation should be done in order
    let nmk_home = NmkHome::find_for_install().expect("Failed to locate NMK_HOME");
    assert!(!nmk_home.is_git(), "nmk is managed by git. Abort.");
    if cmd_opt.backup {
        let home = home_dir().expect("Failed to find home directory");
        let output_tar = home.join("nmk-backup.tar");
        backup_files(&nmk_home, &output_tar)?;
    }
    dotfiles::install_or_update(&cmd_opt, &nmk_home).await?;
    if platform::is_mac() {
        log::error!("Not supporting os");
        return Ok(());
    }
    let entrypoint_updated = entrypoint::install_or_update(&cmd_opt, &nmk_home).await?;
    updater::self_setup(&nmk_home, is_init(), entrypoint_updated).await?;
    if cmd_opt.vendor {
        vendor::install(&cmd_opt, &nmk_home).await?;
    }
    Ok(())
}

fn main() -> nmk::Result<()> {
    let cmd_opt = cmdline::from_args();
    let config = config::Config::new();
    logging::setup(cmd_opt.verbosity);
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    rt.block_on(main_task(cmd_opt, config))
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
        .expect("Failed to find current executable file name")
        .as_bytes()
        .starts_with(b"nmkup-init")
}
