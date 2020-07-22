use std::env;

use structopt::StructOpt;

use nmk::home::NmkHome;
use nmk::platform::is_mac;
use std::path::Path;

mod backup;
mod build;
mod cmdline;
mod dotfiles;
mod entrypoint;
mod logging;
mod os_release;
mod settings;
mod updater;
mod vendor;

async fn main_task(opt: cmdline::Opt, _settings: settings::Settings) -> nmk::Result<()> {
    // Installation should be done in order
    let nmk_home = NmkHome::find_for_install().expect("Unable to locate NMK_HOME");
    assert!(!nmk_home.is_git(), "NMK_HOME is managed by git. Abort.");
    if opt.backup {
        backup::backup_files(&nmk_home);
    }
    dotfiles::install_or_update(&opt, &nmk_home).await?;
    if !is_mac() {
        let entrypoint_updated = entrypoint::install_or_update(&opt, &nmk_home).await?;
        updater::self_setup(&nmk_home, is_nmkup_init(), entrypoint_updated).await?;
        if opt.vendor {
            vendor::install(&opt, &nmk_home).await?;
        }
    }
    Ok(())
}

fn main() -> nmk::Result<()> {
    let opt = cmdline::Opt::from_args();
    let settings = settings::Settings::new(&opt);
    logging::setup(opt.verbosity);
    let mut rt = tokio::runtime::Builder::new()
        .basic_scheduler()
        .enable_all()
        .build()?;
    rt.block_on(main_task(opt, settings))
}

fn is_nmkup_init() -> bool {
    current_exec_stem().as_str().starts_with("nmkup-init")
}

fn current_exec_stem() -> String {
    env::args()
        .next()
        .map(std::path::PathBuf::from)
        .as_deref()
        .and_then(Path::file_stem)
        .and_then(std::ffi::OsStr::to_str)
        .map(String::from)
        .expect("Unable to parse argv[0] as String")
}
