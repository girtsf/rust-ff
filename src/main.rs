extern crate clap;
extern crate colored;
extern crate isatty;
extern crate regex;
extern crate walkdir;

use clap::{App, Arg};
use colored::Colorize;
use regex::{Captures, RegexBuilder};

fn ff(case_sensitive: bool, pattern: Option<&str>) {
    let output_is_tty = isatty::stdout_isatty();
    let re = if let Some(re) = pattern {
        Some(
            RegexBuilder::new(re)
                .case_insensitive(!case_sensitive)
                .build()
                .unwrap(),
        )
    } else {
        None
    };
    let walker = walkdir::WalkDir::new(".");
    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let name = entry.path().to_string_lossy();
        let mut pretty = if let Some(ref re) = re {
            // If regexp is given, skip over stuff that doesn't match.
            if !re.is_match(&name) {
                continue;
            }
            // Only colorize if output is a tty.
            if output_is_tty {
                re.replace_all(&name, |x: &Captures| format!("{}", x[0].red()))
            } else {
                name
            }
        } else {
            name
        };
        if entry.file_type().is_dir() {
            pretty += "/";
        }
        println!("{}", pretty);
    }
}

fn main() {
    let matches = App::new("ff")
        .arg(
            Arg::with_name("case")
                .short("s")
                .long("case-sensitive")
                .help("Case sensitive matching"),
        )
        .arg(Arg::with_name("pattern").index(1))
        .get_matches();

    let case_sensitive = matches.is_present("case");
    let pattern = matches.value_of("pattern");
    ff(case_sensitive, pattern);
}
