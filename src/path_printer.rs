use ansi_term::Colour::Green;
use atty::Stream;
use regex::Regex;

use crate::app;

pub struct PathPrinter<'a> {
    path: String,
    reg_exp: &'a Regex,
}

impl<'a> PathPrinter<'a> {
    pub fn new(path: String, reg_exp: &Regex) -> PathPrinter {
        PathPrinter { path, reg_exp }
    }

    pub fn print(&self) {
        let path = self.truncate_working_dir_path();

        if atty::isnt(Stream::Stdout) {
            self.print_to_non_tty(path);
        } else {
            self.print_to_tty(path);
        }
    }

    fn print_to_non_tty(&self, path: String) {
        println!("{}", path);
    }

    fn print_to_tty(&self, path: String) {
        match self.reg_exp.find(&path) {
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

    fn truncate_working_dir_path(&self) -> String {
        let working_dir_path = app::working_dir_path();

        if self.path.contains(&working_dir_path) {
            self.path.replace(&working_dir_path, ".")
        } else {
            self.path.clone()
        }
    }
}
