use std::fs::File;
use std::io::Write;

use structopt::clap::Shell;
use structopt::StructOpt;

use crate::cmdline::{Completion, Opt};

pub fn completion(options: &Completion) {
    let mut write: Box<dyn Write> = match options.output {
        Some(ref p) => Box::new(File::create(p).expect("Cannot open output file")),
        None => Box::new(std::io::stdout()),
    };
    Opt::clap().gen_completions_to("nmk", options.shell.parse::<Shell>().unwrap(), &mut write);
}
