use crate::app;
use crate::args::Args;
use crate::path_printer::PathPrinter;
use ignore::{WalkBuilder, WalkState};
use regex::Regex;
use std::io;
use std::process;

pub struct Walker<'a> {
    args: &'a Args,
}

impl<'a> Walker<'a> {
    pub fn new(args: &Args) -> Walker {
        Walker { args }
    }

    pub fn walk_and_print(&self) {
        use std::sync::mpsc;
        use std::thread;

        let walker = WalkBuilder::new(&self.args.root_path)
            .hidden(self.args.ignore_hidden)
            .git_ignore(self.args.ignore_gitignore)
            .threads(self.args.threads)
            .build_parallel();

        let (tx, rx) = mpsc::channel::<String>();
        let reg_exp = self.args.reg_exp.clone();

        let print_thread = thread::spawn(move || -> io::Result<()> {
            for path in rx.iter() {
                PathPrinter::new(path, &reg_exp).print();
            }
            Ok(())
        });

        walker.run(|| {
            let tx = tx.clone();
            let regex = self.args.reg_exp.clone();

            Box::new(move |path_entry| {
                if let Ok(entry) = path_entry {
                    let path = entry.path().display().to_string();
                    let path = truncate_working_dir_path(path);

                    if is_match(&regex, &path) {
                        match tx.send(path) {
                            Ok(_) => WalkState::Continue,
                            Err(_) => WalkState::Quit,
                        }
                    } else {
                        WalkState::Continue
                    }
                } else {
                    WalkState::Continue
                }
            })
        });

        drop(tx);

        if let Err(err) = print_thread.join().unwrap() {
            if err.kind() != io::ErrorKind::BrokenPipe {
                if let Some(err_msg) = err.into() {
                    eprintln!("{}", err_msg);
                }

                process::exit(1);
            }
        }
    }
}
fn is_match(reg_exp: &Regex, path: &str) -> bool {
    reg_exp.is_match(path)
}

fn truncate_working_dir_path(path: String) -> String {
    let working_dir_path = app::working_dir_path();

    if path.contains(&working_dir_path) {
        path.replace(&working_dir_path, ".")
    } else {
        path.clone()
    }
}
