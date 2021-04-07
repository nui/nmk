use std::fs::File;
use std::io::Write;

use structopt::StructOpt;

use nmk::bin_name::NMK;

use crate::cmdline::{CmdOpt, Completion};

pub fn gen_completion(completion: &Completion) {
    let mut write: Box<dyn Write> = match completion.output {
        Some(ref p) => Box::new(File::create(p).expect("cannot create completion output file")),
        None => Box::new(std::io::stdout()),
    };
    CmdOpt::clap().gen_completions_to(NMK, completion.shell, &mut write);
}
