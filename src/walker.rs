use walkdir::WalkDir;

use crate::app;
use crate::args::Args;
use crate::path_printer::PathPrinter;

pub struct Walker<'a> {
    args: &'a Args,
}

impl<'a> Walker<'a> {
    pub fn new(args: &Args) -> Walker {
        Walker { args }
    }

    pub fn walk_and_print(&self) {
        for path_entry in WalkDir::new(&self.args.root_path).into_iter() {
            if let Ok(entry) = path_entry {
                let path = entry.path().display().to_string();
                let path = self.truncate_working_dir_path(path);

                if self.is_match(&path) {
                    PathPrinter::new(path, &self.args.reg_exp).print();
                }
            }
        }
    }

    fn truncate_working_dir_path(&self, path: String) -> String {
        let working_dir_path = app::working_dir_path();

        if path.contains(&working_dir_path) {
            path.replace(&working_dir_path, ".")
        } else {
            path.clone()
        }
    }

    fn is_match(&self, path: &str) -> bool {
        if self.args.reg_exp.is_match(path) {
            if self.args.ignore_hidden {
                !path.contains("/.")
            } else {
                true
            }
        } else {
            false
        }
    }
}
