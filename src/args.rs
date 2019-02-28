use ansi_term::Colour::Red;
use atty::Stream;
use clap;
use regex::{Regex, RegexBuilder};
use std::path::Path;
use std::process;

use crate::app;

pub struct Args {
    pub reg_exp: Regex,
    pub root_path: String,
    pub ignore_gitignore: bool,
    pub ignore_hidden: bool,
    pub case_sensitive: bool,
}

struct ArgMatchesWrapper {
    matches: clap::ArgMatches<'static>,
}

impl Args {
    pub fn parse() -> Args {
        let args_matches = ArgMatchesWrapper {
            matches: app::app(),
        };

        args_matches.to_args()
    }
}

impl ArgMatchesWrapper {
    fn is_case_sensitive(&self) -> bool {
        self.matches.is_present("case-sensitive")
    }

    fn should_ignore_gitignore_files(&self) -> bool {
        self.matches.is_present("ignore-gitignore")
    }

    fn should_ignore_hidden_files(&self) -> bool {
        self.matches.is_present("ignore-hidden")
    }

    fn search_pattern(&self) -> Regex {
        let pattern = self.matches.value_of("PATTERN").unwrap();
        let raw_pattern = format!(r#"{}"#, pattern).to_string();
        let regex_builder = RegexBuilder::new(&raw_pattern)
            .case_insensitive(!self.is_case_sensitive())
            .build();

        match regex_builder {
            Ok(reg_exp) => reg_exp,

            Err(_) => {
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
        }
    }

    fn root_path(&self) -> String {
        let root_path = self.matches.value_of("ROOT_PATH").unwrap();

        if Path::new(root_path).is_dir() {
            root_path.to_string()
        } else {
            let erroneous_path = if atty::is(Stream::Stderr) {
                Red.paint(root_path).to_string()
            } else {
                String::from(root_path)
            };

            eprintln!(
                "The specified ROOT_PATH {} is either not accessible or is not a directory",
                erroneous_path
            );

            process::exit(1);
        }
    }

    fn to_args(&self) -> Args {
        Args {
            root_path: self.root_path(),
            ignore_hidden: self.should_ignore_hidden_files(),
            ignore_gitignore: self.should_ignore_gitignore_files(),
            case_sensitive: self.is_case_sensitive(),
            reg_exp: self.search_pattern(),
        }
    }
}
