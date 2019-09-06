use ansi_term::Colour::Red;
use atty::Stream;
use clap;
use num_cpus;
use regex::{Regex, RegexBuilder};
use std::cmp;
use std::path::Path;
use std::process;

use crate::app;

pub struct Args {
    pub reg_exp: Regex,
    pub root_path: String,
    pub exclude_directories: bool,
    pub ignore_gitignore: bool,
    pub ignore_hidden: bool,
    pub case_sensitive: bool,
    pub level: Option<usize>,
    pub threads: usize,
    pub exclude_reg_exp: Option<Regex>,
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

    fn should_exclude_directories(&self) -> bool {
        self.matches.is_present("exclude-directories")
    }

    fn should_ignore_gitignore_files(&self) -> bool {
        self.matches.is_present("ignore-gitignore")
    }

    fn should_ignore_hidden_files(&self) -> bool {
        self.matches.is_present("ignore-hidden")
    }

    fn search_pattern(&self) -> Regex {
        self.parse_regex_from_pattern_of("PATTERN", "Failed to parse the provided PATTERN:")
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

    fn level(&self) -> Option<usize> {
        let matches = &self.matches;

        clap::value_t!(matches.value_of("level"), usize).ok()
    }

    fn threads(&self) -> usize {
        let matches = &self.matches;
        let threads = clap::value_t!(matches.value_of("threads"), usize).unwrap_or(0);

        if threads == 0 {
            cmp::min(12, num_cpus::get())
        } else {
            threads
        }
    }

    fn exclude_reg_exp(&self) -> Option<Regex> {
        if self.matches.is_present("exclude") {
            let reg_exp = self.parse_regex_from_pattern_of(
                "exclude",
                "Failed to parse the pattern provided to the '--exclude (-x)' option:",
            );

            Some(reg_exp)
        } else {
            None
        }
    }

    fn parse_regex_from_pattern_of(&self, arg_name: &str, error_message: &str) -> Regex {
        let input_pattern = self.matches.value_of(arg_name).unwrap();
        let formatted_pattern = format!(r#"{}"#, input_pattern).to_string();
        let regex_builder = RegexBuilder::new(&formatted_pattern)
            .case_insensitive(!self.is_case_sensitive())
            .build();

        match regex_builder {
            Ok(reg_exp) => reg_exp,

            Err(_) => {
                let erroneous_pattern = if atty::is(Stream::Stderr) {
                    Red.paint(formatted_pattern).to_string()
                } else {
                    formatted_pattern
                };

                eprintln!("{} {}", error_message, erroneous_pattern);

                process::exit(1);
            }
        }
    }

    fn to_args(&self) -> Args {
        Args {
            root_path: self.root_path(),
            exclude_directories: self.should_exclude_directories(),
            ignore_hidden: self.should_ignore_hidden_files(),
            ignore_gitignore: self.should_ignore_gitignore_files(),
            case_sensitive: self.is_case_sensitive(),
            reg_exp: self.search_pattern(),
            level: self.level(),
            threads: self.threads(),
            exclude_reg_exp: self.exclude_reg_exp(),
        }
    }
}
