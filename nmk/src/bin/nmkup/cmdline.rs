use structopt::StructOpt;

#[structopt(name = "nmkup", about = "The NMK installer/updater")]
#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(short, long, help = "Force install")]
    pub force: bool,
    #[structopt(short, long, help = "Backup important files before update")]
    pub backup: bool,
    #[structopt(long, help = "Do not filter items based on /etc/os-release data")]
    pub no_filter: bool,
    #[structopt(long, help = "Install vendored files")]
    pub vendor: bool,
    #[structopt(short, parse(from_occurrences), help = "Request verbose logging")]
    pub verbosity: u8,
}
