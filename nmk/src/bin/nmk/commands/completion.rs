use std::fs::File;
use std::io::Write;

use structopt::StructOpt;

use nmk::consts::bin::NMK;

use crate::cmdline::{CmdOpt, Completion};

pub fn generate_completion(completion: Completion) {
    let mut output: Box<dyn Write> = match completion.output {
        Some(ref p) => Box::new(File::create(p).expect("cannot create completion file")),
        None => Box::new(std::io::stdout()),
    };
    CmdOpt::clap().gen_completions_to(NMK, completion.shell, &mut output);
}
