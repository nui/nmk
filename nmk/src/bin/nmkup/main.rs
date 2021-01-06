use std::path::Path;

use structopt::StructOpt;

use nmk::home::NmkHome;
use nmk::platform::is_mac;

mod backup;
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
    let nmk_home = NmkHome::find_for_install().expect("Unable to locate NMK_HOME");
    assert!(!nmk_home.is_git(), "nmk is managed by git. Abort.");
    if cmd_opt.backup {
        backup::backup_files(&nmk_home)?;
    }
    dotfiles::install_or_update(&cmd_opt, &nmk_home).await?;
    if !is_mac() {
        let entrypoint_updated = entrypoint::install_or_update(&cmd_opt, &nmk_home).await?;
        updater::self_setup(&nmk_home, is_nmkup_init(), entrypoint_updated).await?;
        if cmd_opt.vendor {
            vendor::install(&cmd_opt, &nmk_home).await?;
        }
    }
    Ok(())
}

fn main() -> nmk::Result<()> {
    let cmd_opt = cmdline::CmdOpt::from_args();
    let settings = config::Config::new();
    logging::setup(cmd_opt.verbosity);
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    rt.block_on(main_task(cmd_opt, settings))
}

fn is_nmkup_init() -> bool {
    current_exec_name().starts_with("nmkup-init")
}

fn current_exec_name() -> String {
    std::env::current_exe()
        .ok()
        .as_deref()
        .and_then(Path::file_name)
        .and_then(std::ffi::OsStr::to_str)
        .map(String::from)
        .expect("Unable to parse argv[0] as String")
}
