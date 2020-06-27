use structopt::StructOpt;

#[structopt(name = "nmkup", about = "All in one binary to setup nmk")]
#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(short, parse(from_occurrences), help = "Request verbose logging")]
    pub verbosity: u8,
    #[structopt(long, help = "Install vendored files")]
    pub vendor: bool,
    #[structopt(short, long, help = "Force install")]
    pub force: bool,
}
