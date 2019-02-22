extern crate ansi_term;
extern crate clap;
extern crate regex;
extern crate walkdir;

use ansi_term::Colour::Red;
use clap::{crate_authors, crate_version, App};
use regex::Regex;
use std::process;
use walkdir::{WalkDir};

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
                let ok_entries = walker.filter_map(|e| e.ok());
                for entry in ok_entries
                    .filter(|e| regex_pattern.is_match(e.path().display().to_string().as_str()))
                {
                    println!("{}", entry.path().display());
                }
            }
            Err(_err) => {
                eprintln!(
                    "Failed to parse the provided PATTERN: {}",
                    formatted_pattern
                );
                process::exit(1);
            }
        }
    }
}
