/// Main

use std::path::Path;
use std::collections::HashSet;
use std::path::PathBuf;
use sixarm_collections::*;
use walkdir::WalkDir;

mod args;
mod assure;
mod clap;
mod util;

use args::Args;
use util::{file_bytes_eq};

//extern crate custom_error;
//use custom_error::custom_error;

/// File length
pub type FileLen = u64;

/// Set of file path buffers
pub type SetOfPathBuf = HashSet<PathBuf>;

/// How to track files
pub type Tracker = HashMapOfFileLenToSetOfPathBuf;

/// Process one path, by walking the path then calling `on_file`.
pub fn on_path<T: AsRef<Path>>(args: Args, tracker: &mut Tracker, path: T) -> () {
    if args.verbose > 0 { println!("on_path path:{:?}", path.as_ref())}
    for entry in WalkDir::new(path.as_ref())
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().is_file()) {
        on_file(args, tracker, entry.path());
    }
}

/// Process one file, by deciding its relevance and how to handle it.
pub fn on_file<T: AsRef<Path>>(args: Args, tracker: &mut Tracker, path: T) -> () {
    if args.verbose > 0 { println!("on_file path:{}", path.as_ref().display())}
    if args.print {
        println!("{}", path.as_ref().display());
    }
    tracker.insert_path(path.as_ref().to_path_buf());
}

pub fn on_clone<T: AsRef<Path>>(args: Args, tracker: &mut Tracker, a_path: T, b_path: T) {
    if args.verbose > 0 { println!("on_clone path:{:?} path:{:?}", path.as_ref())}
    println!("Clone: {:?} {:?}", a_path.as_ref(), b_path.as_ref());
    match std::fs::remove_file(&b_path) {
        Ok(()) => (),
        Err(e) => eprintln!("err:{} remove_file:{:?}", e, b_path.as_ref().display()),
    }
}

pub fn detect_duplicates(a_tracker: Tracker, b_tracker: Tracker) {
    for len in b_tracker.map.keys() {
        if !a_tracker.map.contains_key(len) { continue }
        let a_paths: &SetOfPathBuf = a_tracker.map.get(len).unwrap();
        let b_paths: &SetOfPathBuf = b_tracker.map.get(len).unwrap();
        'outer: for b_path in b_paths.iter() {
            'inner: for a_path in a_paths.iter() {
                if same_file::is_same_file(a_path, b_path).unwrap() { 
                    continue 'inner
                }
                match file_bytes_eq(&a_path, &b_path) {
                    Ok(flag) => {
                        if flag {
                            on_duplicate_file(a_path, b_path);
                            continue 'outer;
                        }
                    },
                    Err(e) => println!("{:?}", e),
                }
            }
        }
    }
}


fn main() {
    let args = clap::args();
    if args.verbose > 1 {
        println!("verbose level:{}", args.verbose);
        if args.find_cloned { println!("--find-cloned"); }
        if args.find_clones { println!("--find-clones"); }
        if args.find_uniques { println!("--find-uniques"); }
        if args.print { println!("--print"); }
        if args.delete { println!("--delete"); }
        if args.recycle { println!("--recycle"); }
        if args.shred { println!("--shred"); }
        if args.symlink { println!("--symlink"); }
        if args.hardlink { println!("--hardlink"); }
    }

    let mut tracker = Tracker::new();

    args.paths.iter().for_each(|path| {
        println!("{:?}", path);
        tracker.insert_path(path.to_owned());
    });

    // detect_duplicates(&path_a_tracker, &path_b_tracker)
}

#[cfg(test)]
//#[macro_use] extern crate assert_matches;
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    /// Test `on_path` via one directory that contains these files:
    /// 
    /// * `alpha.txt` which contains `alpha` (len == 6)
    /// * `bravo.txt` which contains `bravo` (len == 6)
    /// * `charlie.txt` which contains `charlie` (len == 8)
    /// 
    fn test_on_path() {
        let args = Args {};
        let mut tracker = Tracker::new();
        let path_buf: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "main", "on_path"].iter().collect();
        let path_buf_alpha: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "main", "on_path", "alpha.txt"].iter().collect();
        let path_buf_bravo: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "main", "on_path", "bravo.txt"].iter().collect();
        let path_buf_charlie: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "main", "on_path", "charlie.txt"].iter().collect();
        on_path(args, &mut tracker, path_buf);
        assert!(tracker.contains_path(&path_buf_alpha));
        assert!(tracker.contains_path(&path_buf_bravo));
        assert!(tracker.contains_path(&path_buf_charlie));
    }

    #[test]
    /// Test `on_file` via one file.
    fn test_on_file() {
        let args = Args {};
        let mut tracker = Tracker::new();
        let path_buf: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "main", "on_path", "alpha.txt"].iter().collect();
        on_file(args, &mut tracker, path_buf);
        assert!(tracker.contains_path(&path_buf));
    }

    #[test]
    /// Test `print_duplicates` via these files:
    /// 
    /// * `a/alpha.txt` which contains `alpha`
    /// * `a/bravo.txt` which contains `bravo`
    /// * `b/alpha.txt` which contains `alpha`
    /// * `b/bravo.txt` which contains `bravo`
    /// 
    fn test_detect_duplicates() {
        let a_path_buf: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "print_duplicates", "a"].iter().collect();
        let b_path_buf: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "print_duplicates", "b"].iter().collect();
        let mut a_tracker = Tracker::new();
        let mut b_tracker = Tracker::new();
        on_path(&mut a_tracker, a_path_buf);
        on_path(&mut b_tracker, b_path_buf);
        //TODO
        // detect_duplicates(&a_tracker, &b_tracker);
    }

}
