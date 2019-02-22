extern crate ansi_term;
extern crate atty;
extern crate clap;
extern crate regex;
extern crate walkdir;

use ansi_term::Colour::{Green, Red};
use atty::Stream;
use clap::{crate_authors, crate_version, App};
use regex::Regex;
use std::process;
use walkdir::{DirEntry, WalkDir};

fn main() {
    let matches = App::new("ff")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Find Files. Yea. Period.")
        .max_term_width(80)
        .args_from_usage("<PATTERN> 'Search for files matching file name with this pattern'")
        .get_matches();

    if let Some(pattern) = matches.value_of("PATTERN") {
        let raw_pattern = format!(r#"{}"#, pattern).to_string();

        match Regex::new(&raw_pattern) {
            Ok(reg_exp) => lookup(reg_exp),
            Err(_) => handle_erroneous_pattern(raw_pattern),
        }
    }
}

fn lookup(reg_exp: Regex) {
    let root_path = ".";
    let paths = accessible_paths(root_path);

    for entry in matching_paths(paths, reg_exp.clone()) {
        let file_path = entry.path().display().to_string();

        print_path(file_path, reg_exp.clone());
    }
}

fn accessible_paths(root_path: &str) -> impl Iterator<Item = DirEntry> {
    let walker = WalkDir::new(root_path).into_iter();

    walker.filter_map(|e| e.ok())
}

fn matching_paths(
    paths: impl Iterator<Item = DirEntry>,
    reg_exp: Regex,
) -> impl Iterator<Item = DirEntry> {
    paths.filter(move |e| reg_exp.is_match(&e.path().display().to_string()))
}

fn print_path(path: String, reg_exp: Regex) {
    if atty::isnt(Stream::Stdout) {
        println!("{}", path);
    } else {
        match reg_exp.find(&path) {
            Some(result) => {
                let matched_str = &path[result.start()..result.end()];
                let colored_match = Green.bold().paint(matched_str).to_string();
                let path = &path.replace(matched_str, &colored_match);
                println!("{}", path);
            }
            None => {
                println!("{}", path);
            }
        }
    }
}

fn handle_erroneous_pattern(raw_pattern: String) {
    let erroneous_pattern = if atty::is(Stream::Stderr) {
        Red.paint(raw_pattern).to_string()
    } else {
        raw_pattern
    };

    eprintln!(
        "Failed to parse the provided PATTERN: {}",
        erroneous_pattern
    );

    process::exit(1);
}
