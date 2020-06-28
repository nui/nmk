use structopt::StructOpt;

use nmk::home::NmkHome;
use nmk::platform::is_mac;

mod build;
mod cmdline;
mod dotfiles;
mod entrypoint;
mod logging;
mod settings;
mod updater;
mod vendor;

pub const ARTIFACT_BASE_URL: &str = "https://storage.googleapis.com/nmk.nuimk.com";

async fn main_task(opt: cmdline::Opt, _settings: settings::Settings) -> nmk::Result<()> {
    // Installation should be done in order
    let nmk_home = NmkHome::find().expect("Unable to locate NMK_HOME");
    assert!(!nmk_home.is_git(), "NMK_HOME is managed by git. Abort.");
    dotfiles::install_or_update(&opt, &nmk_home).await?;
    if !is_mac() {
        let entrypoint_updated = entrypoint::install_or_update(&opt, &nmk_home).await?;
        updater::self_setup(&nmk_home, is_nmkup_init(), entrypoint_updated).await?;
        if opt.vendor {
            vendor::install(&nmk_home).await?;
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
    std::env::args()
        .next()
        .map(std::path::PathBuf::from)
        .as_ref()
        .and_then(|a| a.file_stem())
        .and_then(std::ffi::OsStr::to_str)
        .map(String::from)
        .expect("Unable to parse argv[0] as String")
}
