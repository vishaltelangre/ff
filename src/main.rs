extern crate ansi_term;
extern crate atty;
extern crate clap;
extern crate lazy_static;
extern crate regex;
extern crate walkdir;

use crate::args::Args;
use crate::path_printer::PathPrinter;
use crate::walker::Walker;

mod app;
mod args;
mod path_printer;
mod walker;

fn main() {
    let args = Args::parse();

    for path_item in Walker::new(&args).matching_paths() {
        let path = path_item.path().display().to_string();

        PathPrinter::new(path, &args.reg_exp).print()
    }
}
