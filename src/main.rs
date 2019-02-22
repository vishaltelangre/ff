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
use walkdir::WalkDir;

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
        match Regex::new(&formatted_pattern) {
            Ok(regex_pattern) => {
                let walker = WalkDir::new(".").into_iter();
                let ok_entries = walker.filter_map(|e| e.ok());
                for entry in
                    ok_entries.filter(|e| regex_pattern.is_match(&e.path().display().to_string()))
                {
                    let file_path = entry.path().display().to_string();

                    if atty::isnt(Stream::Stdout) {
                        println!("{}", file_path);

                        process::exit(0);
                    }

                    match regex_pattern.find(&file_path) {
                        Some(result) => {
                            let matched_slice = &file_path[result.start()..result.end()];
                            let colored_match = Green.bold().paint(matched_slice).to_string();
                            let file_path = &file_path.replace(matched_slice, &colored_match);
                            println!("{}", file_path);
                        }
                        None => {
                            println!("{}", file_path);
                        }
                    }
                }
            }
            Err(_) => {
                let erroneous_pattern = if atty::is(Stream::Stderr) {
                    Red.paint(formatted_pattern).to_string()
                } else {
                    formatted_pattern
                };

                eprintln!(
                    "Failed to parse the provided PATTERN: {}",
                    erroneous_pattern
                );

                process::exit(1);
            }
        }
    }
}
