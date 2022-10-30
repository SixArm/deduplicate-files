//! clap setup.
//!
//! clap is a crate for command line argument parsing.
//! See https://docs.rs/clap/
//!
//! We prefer clap using the `command!` macro, which runs at compile time.
//! We prefer clap using the builder pattern, which offers more capabilties.
//!
//! We favor our convention of doing clap setup in a file named `clap.rs`,
//! rather than in `main.rs`, because we favor the separation of concerns.

use clap::{Arg, ArgAction};
use crate::args::Args;

use std::path::PathBuf;
use crate::args::Args;

/// Create a clap command.
pub fn clap() -> crate::Args {
    let matches = clap::command!()
    .name("deduplicate-files")
    .version("1.0.0")
    .author("Joel Parker Henderson <joel@joelparkerhenderson.com>")
    .help("Helps with file synchronization, dedupilication, and more")
    .arg(Arg::new("verbose")
        .help("Set the verbosity level")
        .short('v')
        .long("verbose")
        .action(ArgAction::Count))
    .arg(Arg::new("find-cloned")
        .help("Find files that are cloned i.e. originals (compare --find-clones, --find-uniques)")
        .long("find-cloned"))
    .arg(Arg::new("find-clones")
        .help("Find files that are clones i.e. duplicates (compare --find-cloned, --find-uniques)")
        .long("find-clones"))
    .arg(Arg::new("find-uniques")
        .help("Find files that are uniques i.e not cloned or clones (compare --find-cloned, --find-clones)")
        .long("find-uniques"))
    .arg(Arg::new("print")
        .help("Print the results i.e. dry run")
        .long("print"))
    .arg(Arg::new("delete")
        .help("Delete clones immediately")
        .long("delete")
        .requires("find-clones")
        .conflicts_with("recycle")
        .conflicts_with("shred")
        .conflicts_with("symlink")
        .conflicts_with("hardlink"))
    .arg(Arg::new("recycle")
        .help("Recycle clones to the trash folder")
        .long("recycle")
        .requires("find-clones")
        .conflicts_with("delete")
        .conflicts_with("shred")
        .conflicts_with("symlink")
        .conflicts_with("hardlink"))
    .arg(Arg::new("shred")
        .help("Shred clones by secure overwrite")
        .long("shred")
        .requires("find-clones")
        .conflicts_with("delete")
        .conflicts_with("recycle")
        .conflicts_with("symlink")
        .conflicts_with("hardlink"))
    .arg(Arg::new("symlink")
        .help("Symlink clones to their originals")
        .long("symlink")
        .requires("find-clones")
        .conflicts_with("delete")
        .conflicts_with("recycle")
        .conflicts_with("shred")
        .conflicts_with("hardlink"))
    .arg(Arg::new("hardlink")
        .help("Hardlink clones to their originals")
        .long("hardlink")
        .requires("find-clones")
        .conflicts_with("delete")
        .conflicts_with("recycle")
        .conflicts_with("shred")
        .conflicts_with("symlink"))
    .arg(Arg::new("paths")
        .help("Paths to process")
        .min_values(0));
    Args {
        verbose: std::cmp::max(3, matches.occurrences_of("verbose") as u8),
        find_clones: matches.is_present("find-clones"),
        find_cloned: matches.is_present("find-cloned"),
        find_uniques: matches.is_present("find-uniques"),
        print: matches.is_present("print"),
        delete: matches.is_present("delete"),
        recycle: matches.is_present("recycle"),
        shred: matches.is_present("shred"),
        symlink: matches.is_present("symlink"),
        hardlink: matches.is_present("hardlink"),
        paths: matches.values_of("paths")
        .unwrap_or_else(||Values::default())
        .map(|x| PathBuf::from(x)).collect(),
    }
}
