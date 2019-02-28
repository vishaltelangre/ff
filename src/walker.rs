use walkdir::{DirEntry, WalkDir};

use crate::args::Args;
use crate::app;

pub struct Walker<'a> {
    args: &'a Args,
}

impl<'a> Walker<'a> {
    pub fn new(args: &Args) -> Walker {
        Walker { args }
    }

    fn accessible_paths(&self) -> Vec<DirEntry> {
        let iterator = WalkDir::new(&self.args.root_path).into_iter();

        iterator.filter_map(|e| e.ok()).collect()
    }

    pub fn matching_paths(&self) -> Vec<String> {
        let paths: Vec<String> = self
            .accessible_paths()
            .into_iter()
            .map(|p| self.truncate_working_dir_path(p.path().display().to_string()))
            .filter(|path| self.args.reg_exp.is_match(path))
            .collect();

        if self.args.ignore_hidden {
            paths
                .into_iter()
                .filter(|path| !path.contains("/."))
                .collect()
        } else {
            paths
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
}
