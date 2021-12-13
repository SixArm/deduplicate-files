/// clap setup.
///
/// clap is a crate for command line argument parsing.
/// See https://docs.rs/clap/
///
/// Clap has a variety of setup approachs:
///
///   * via typical functions, which favors advanced uses yet is verbose.
///   * via usage strings, which looks more like writing documentation.
///   * via macros, which is fast and less verbose, yet atypical to read.
///   * via YAML file, which favors localization and text file readability.
///
/// We prefer the typical functions, because they provide maximum capability,
/// and in our experience are the easiest for Rust IDEs to read and debug.
///
/// We favor our convention of doing clap setup in a file named `clap.rs`,
/// rather than in `main.rs`, because we like the separation of concerns.

use clap::{Arg,App,Values};
use std::path::PathBuf;
use crate::args::Args;

/// Create a clap app.
pub fn app() -> App<'static> {
    App::new("FileSync")
    .version("1.0.0")
    .author("Joel Parker Henderson <joel@joelparkerhenderson.com>")
    .about("Helps with file synchronization, dedupilication, and more")
    .arg(Arg::new("verbose")
        .short('v')
        .long("verbose")
        .multiple(true)
        .about("Set the verbosity level"))
    .arg(Arg::new("find-cloned")
        .long("find-cloned")
        .about("Find files that are cloned i.e. originals (compare --find-clones, --find-uniques)"))
    .arg(Arg::new("find-clones")
        .long("find-clones")
        .about("Find files that are clones i.e. duplicates (compare --find-cloned, --find-uniques)"))
    .arg(Arg::new("find-uniques")
        .long("find-uniques")
        .about("Find files that are uniques i.e not cloned or clones (compare --find-cloned, --find-clones)"))
    .arg(Arg::new("print")
        .long("print")
        .about("Print the results i.e. dry run"))
    .arg(Arg::new("delete")
        .long("delete")
        .about("Delete clones immediately")
        .requires("find-clones")
        .conflicts_with("recycle")
        .conflicts_with("shred")
        .conflicts_with("symlink")
        .conflicts_with("hardlink"))
    .arg(Arg::new("recycle")
        .long("recycle")
        .about("Recycle clones to the trash folder")
        .requires("find-clones")
        .conflicts_with("delete")
        .conflicts_with("shred")
        .conflicts_with("symlink")
        .conflicts_with("hardlink"))
    .arg(Arg::new("shred")
        .long("shred")
        .about("Shred clones by secure overwrite")
        .requires("find-clones")
        .conflicts_with("delete")
        .conflicts_with("recycle")
        .conflicts_with("symlink")
        .conflicts_with("hardlink"))
    .arg(Arg::new("symlink")
        .long("symlink")
        .about("Symlink clones to their originals")
        .requires("find-clones")
        .conflicts_with("delete")
        .conflicts_with("recycle")
        .conflicts_with("shred")
        .conflicts_with("hardlink"))
    .arg(Arg::new("hardlink")
        .long("hardlink")
        .about("Hardlink clones to their originals")
        .requires("find-clones")
        .conflicts_with("delete")
        .conflicts_with("recycle")
        .conflicts_with("shred")
        .conflicts_with("symlink"))
    .arg(Arg::new("paths")
        .multiple(true))
}
/// Create an Args struct initiatied with the clap App settings.
pub fn args() -> Args {
    let matches = app().get_matches();
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
