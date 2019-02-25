use walkdir::{DirEntry, WalkDir};

use crate::args::Args;

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

    pub fn matching_paths(&self) -> Vec<DirEntry> {
        let paths = self
            .accessible_paths()
            .into_iter()
            .filter(|p| self.args.reg_exp.is_match(&p.path().display().to_string()))
            .collect();

        if self.args.search_hidden {
            paths
        } else {
            paths
                .into_iter()
                .filter(|p| !p.path().display().to_string().contains("/."))
                .collect()
        }
    }
}
