extern crate ansi_term;
extern crate atty;
extern crate clap;
extern crate lazy_static;
extern crate ignore;
extern crate regex;
extern crate walkdir;

use crate::args::Args;
use crate::walker::Walker;

mod app;
mod args;
mod path_printer;
mod walker;

fn main() {
    let args = Args::parse();

    Walker::new(&args).walk_and_print();
}
