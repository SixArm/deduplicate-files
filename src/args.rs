/// Args for the application. 
///
/// These args correspond to the matches in the file `clap.rs`.
/// We have these args in their own file in order to be flexible,
/// such as being able to start our app with other arg parsers.

use std::default::Default;
use std::path::PathBuf;

#[derive(Default, Debug)]
pub struct Args {
    pub(crate) verbose: u8,
    pub(crate) find_clones: bool,
    pub(crate) find_cloned: bool,
    pub(crate) find_uniques: bool,
    pub(crate) print: bool,
    pub(crate) delete: bool,
    pub(crate) recycle: bool,
    pub(crate) shred: bool,
    pub(crate) symlink: bool,
    pub(crate) hardlink: bool,
    pub(crate) paths: Vec<PathBuf>,
}
