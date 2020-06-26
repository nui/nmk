use structopt::StructOpt;

use nmk::home::NmkHome;

mod build;
mod cmdline;
mod dotfiles;
mod entrypoint;
mod logging;
mod settings;
mod updater;

pub const ARTIFACT_BASE_URL: &str = "https://storage.googleapis.com/nmk.nuimk.com";

#[tokio::main]
pub async fn main() -> nmk::Result<()> {
    let opt = cmdline::Opt::from_args();
    let _settings = settings::Settings::new(&opt);
    crate::logging::setup(opt.verbosity);

    // Installation should be done in order
    // let nmk_home = NmkHome::from(dirs::home_dir().unwrap().join("nmk-testing"));
    let nmk_home = NmkHome::find().expect("Unable to locate NMK_HOME");
    assert!(!nmk_home.is_git(), "NMK_HOME is git");
    dotfiles::install_or_update(&opt, &nmk_home).await?;
    let entrypoint_updated = entrypoint::install_or_update(&opt, &nmk_home).await?;
    updater::self_setup(&nmk_home, is_nmkup_init(), entrypoint_updated).await?;
    Ok(())
}

fn is_nmkup_init() -> bool {
    matches!(current_exec_name().as_str(), "nmkup-init")
}

fn current_exec_name() -> String {
    std::env::args()
        .next()
        .map(std::path::PathBuf::from)
        .as_ref()
        .and_then(|a| a.file_stem())
        .and_then(std::ffi::OsStr::to_str)
        .map(String::from)
        .expect("Unable to parse argv[0] as String")
}
