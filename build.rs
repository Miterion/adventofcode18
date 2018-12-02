extern crate clap;

use std::env;
use clap::Shell;
use std::fs::File;

include!("src/cli.rs");

fn main() {
    let outdir = match env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };
    let mut app = build_cli();
    let mut f = File::create("help").unwrap();
    app.write_long_help(&mut f).unwrap();
    app.gen_completions("advent",      // We need to specify the bin name manually
                        Shell::Zsh,  // Then say which shell to build completions for
                        outdir);      // Then say where write the completions to
}