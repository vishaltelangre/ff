use ansi_term::Colour::Green;
use atty::Stream;
use regex::Regex;

pub struct PathPrinter<'a> {
    path: String,
    reg_exp: &'a Regex,
}

impl<'a> PathPrinter<'a> {
    pub fn new(path: String, reg_exp: &Regex) -> PathPrinter {
        PathPrinter { path, reg_exp }
    }

    pub fn print(&self) {
        if atty::isnt(Stream::Stdout) {
            self.print_to_non_tty();
        } else {
            self.print_to_tty();
        }
    }

    fn print_to_non_tty(&self) {
        println!("{}", self.path);
    }

    fn print_to_tty(&self) {
        match self.reg_exp.find(&self.path) {
            Some(result) => {
                let matched_str = &self.path[result.start()..result.end()];
                let colored_match = Green.bold().paint(matched_str).to_string();
                let path = &self.path.replace(matched_str, &colored_match);

                println!("{}", path);
            }

            None => {
                println!("{}", self.path);
            }
        }
    }
}
