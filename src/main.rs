extern crate clap;
extern crate walkdir;
extern crate ansi_term;
extern crate regex;

use clap::{crate_authors, crate_version, App};
use walkdir::WalkDir;
use ansi_term::Colour::Red;
use regex::{Regex};
use std::process;

fn main() {
    let matches = App::new("ff")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Find Files. Yea. Period.")
        .max_term_width(80)
        .args_from_usage("<PATTERN> 'Search for files matching file name with this pattern'")
        .get_matches();

    if let Some(pattern) = matches.value_of("PATTERN") {
        let formatted_pattern = format!(r#"{}"#, pattern).to_string();
        match Regex::new(formatted_pattern.as_str()) {
            Ok(regex_pattern) => {
                let walker = WalkDir::new(".").into_iter();
                for entry in walker.filter_map(|e| e.ok()) {
                    if regex_pattern.is_match(entry.path().display().to_string().as_str()) {
                        println!("{}", entry.path().display());
                    }
                }
            },
            Err(err) => {
                println!("Failed to parse the provided PATTERN: {}", formatted_pattern);
                process::exit(1);
            }
        }
    }
}
