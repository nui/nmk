use once_cell::sync::Lazy;
use structopt::StructOpt;

static VERSION: Lazy<String> = Lazy::new(|| ::common::get_version().unwrap_or_default());

#[derive(Debug, StructOpt)]
#[structopt(name = "nmkup", about = "All in one binary to setup nmk", version = VERSION.as_str())]
pub struct Opt {
    #[structopt(short, long, help = "Display debug log")]
    pub debug: bool,
    #[structopt(short, long, help = "Force install")]
    pub force: bool,
}
