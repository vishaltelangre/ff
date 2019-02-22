extern crate ansi_term;
extern crate atty;
extern crate clap;
extern crate lazy_static;
extern crate regex;
extern crate walkdir;

use ansi_term::Colour::{Green, Red};
use atty::Stream;
use clap::{crate_authors, crate_version, App, Arg, ArgMatches};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use std::env;
use std::path::Path;
use std::process;
use walkdir::{DirEntry, WalkDir};

const ABOUT: &str = "
Find Files (ff) utility recursively searches the files whose names match the
specified RegExp pattern in the provided directory (defaults to the current
directory if not provided).";

fn main() {
    let args = parse_args();

    let pattern = args.value_of("PATTERN").unwrap();
    let root_path = args.value_of("ROOT_PATH").unwrap();
    let search_hidden = args.is_present("search-hidden");
    let case_sensitive = args.is_present("case-sensitive");

    ensure_root_path_is_walkable(root_path);

    let raw_pattern = format!(r#"{}"#, pattern).to_string();
    let regex_builder = RegexBuilder::new(&raw_pattern)
        .case_insensitive(!case_sensitive)
        .build();

    match regex_builder {
        Ok(reg_exp) => lookup(root_path, reg_exp, search_hidden),
        Err(_) => handle_erroneous_pattern(raw_pattern),
    }
}

fn parse_args() -> ArgMatches<'static> {
    lazy_static! {
        static ref WORKING_DIR_PATH: String = working_dir_path();
    }

    App::new("ff")
        .version(crate_version!())
        .author(crate_authors!())
        .about(ABOUT)
        .max_term_width(80)
        .arg(
            Arg::with_name("PATTERN")
                .help("Find files whose name (path) matches this substring or the regular expression.")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("ROOT_PATH")
                .help("Path to the directory to search files inside.")
                .index(2)
                .default_value(&WORKING_DIR_PATH)
                .required(false),
        )
        .arg(
            Arg::with_name("search-hidden")
                .help("Search hidden files and directories. By default, hidden files and directories are skipped.")
                .short("H")
                .long("search-hidden"),
        )
        .arg(
            Arg::with_name("case-sensitive")
                .help("Search case sensitively. By default, files are searched case insensitively.")
                .short("s")
                .long("case-sensitive"),
        )
        .get_matches()
}

fn ensure_root_path_is_walkable(path: &str) {
    let erroneous_path = if atty::is(Stream::Stderr) {
        Red.paint(path).to_string()
    } else {
        String::from(path)
    };

    if !Path::new(path).is_dir() {
        eprintln!(
            "The specified ROOT_PATH {} is either not accessible or is not a directory",
            erroneous_path
        );

        process::exit(1);
    }
}

fn working_dir_path() -> String {
    match env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(_) => String::from("."),
    }
}

fn lookup(root_path: &str, reg_exp: Regex, search_hidden: bool) {
    let paths = accessible_paths(root_path);

    for entry in matching_paths(paths, reg_exp.clone(), search_hidden) {
        let file_path = entry.path().display().to_string();

        print_path(file_path, reg_exp.clone());
    }
}

fn accessible_paths(root_path: &str) -> Vec<DirEntry> {
    let walker = WalkDir::new(root_path).into_iter();

    walker.filter_map(|e| e.ok()).collect()
}

fn matching_paths(paths: Vec<DirEntry>, reg_exp: Regex, search_hidden: bool) -> Vec<DirEntry> {
    let paths: Vec<DirEntry> = paths
        .into_iter()
        .filter(|e| reg_exp.is_match(&e.path().display().to_string()))
        .collect();

    if search_hidden {
        paths
    } else {
        paths
            .into_iter()
            .filter(|e| !e.path().display().to_string().contains("/."))
            .collect()
    }
}

fn print_path(path: String, reg_exp: Regex) {
    let working_dir_path = working_dir_path();
    let path = if path.contains(&working_dir_path) {
        path.replace(&working_dir_path, ".")
    } else {
        path
    };

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
