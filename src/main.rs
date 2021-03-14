/// Main

use std::collections::{HashMap,HashSet};
use std::{env,io,fs};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path,PathBuf};
use walkdir::WalkDir;

extern crate custom_error;
use custom_error::custom_error;

/// File length
type Len = u64;

/// Set of file path buffers
type PathSet = HashSet<PathBuf>;

/// Map from file length to path set
type MapLenToPathSet = HashMap<Len, PathSet>;

/// File buffer size
const FILE_BUFFER_SIZE: usize = 4096;

// Custom error that contains a file path
custom_error! {MyError
    VetPathIsDirError {
        path: PathBuf
    } = @{format!("vet_path_is_dir error: {path}", path=path.display())},
    VetPathIsFileError {
        path: PathBuf
    } = @{format!("vet_path_is_file error: {path}", path=path.display())},
}

/// Create a lookup of a directory's files, from a file length to
/// set of files of that length, i.e. map each length to its file paths.
pub fn dir_to_mapper<T: AsRef<Path>>(dir: T) -> MapLenToPathSet {
    let mut mapper: MapLenToPathSet = MapLenToPathSet::new();
    for entry in WalkDir::new(dir.as_ref())
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().is_file()) {
        let metadata = entry.metadata().expect("metadata");
        let len = metadata.len();
        mapper.entry(len).or_insert(PathSet::new()).insert(entry.path().to_owned());
    }
    return mapper
}

// Do two arrays contain equal elements, up to a given length index?
// This function is here in order to compare two file byte stream buffers,
// thus the array index only needs to go up to buffers' lesser fill length.
pub fn array_eq_with_len<'a, T: PartialEq>(a: &'a [T], b: &'a [T], len: usize) -> bool {
    for i in 0..len {
        if a[i] != b[i] {
            return false
        }
    }
    true
}

pub fn vet_path_is_dir<T: AsRef<Path>>(path: T) -> io::Result<bool> {
    match fs::metadata(path) {
        Ok(metadata) => {
            match metadata.is_dir() {
                true => Ok(true),
                false => Err(
                    io::Error::new(
                    io::ErrorKind::InvalidInput, 
                    "vet_path_is_dir",
                    )
                )
            }
        },
        Err(e) => Err(e)
    }
}

pub fn vet_path_is_file<T: AsRef<Path>>(path: T) -> io::Result<bool> {
    match fs::metadata(path) {
        Ok(metadata) => {
            match metadata.is_file() {
                true => Ok(true),
                false => Err(
                    io::Error::new(
                    io::ErrorKind::InvalidInput, 
                    "vet_path_is_file",
                    )
                )
            }
        },
        Err(e) => Err(e)
    }
}

// Do two files have equal length?
pub fn file_len_eq<T: AsRef<Path>>(a: T, b: T) -> io::Result<bool> {
    let a_metadata = fs::metadata(a)?;
    let b_metadata = fs::metadata(b)?;
    match a_metadata.is_file() && b_metadata.is_file() {
        true => Ok(a_metadata.len() == b_metadata.len()),
        false => Err(
            io::Error::new(
            io::ErrorKind::InvalidInput, 
            "file_len_eq",
            )
        )
    }
}

// Do two files have equal bytes?
pub fn file_bytes_eq<T: AsRef<Path>>(a: T, b: T) -> io::Result<bool> {
    let mut a_file = File::open(a)?;
    let mut b_file = File::open(b)?;
    let mut a_buffer = [0; FILE_BUFFER_SIZE];
    let mut b_buffer = [0; FILE_BUFFER_SIZE];
    loop {
        let a_n = a_file.read(&mut a_buffer)?;
        let b_n = b_file.read(&mut b_buffer)?;
        if a_n == 0 && b_n == 0 { return Ok(true) }
        if a_n != b_n { return Ok(false) }
        if !array_eq_with_len(&a_buffer, &b_buffer, a_n) { return Ok(false) }
    }
}

pub fn on_duplicate_file<T: AsRef<Path>>(a: T, b: T) {
    println!("Duplicate: {:?} {:?}", a.as_ref(), b.as_ref());
    match std::fs::remove_file(&b) {
        Ok(()) => (),
        Err(e) => eprintln!("err:{} remove_file:{:?}", e, b.as_ref()),
    }
}

pub fn print_duplicates(a_mapper: &MapLenToPathSet, b_mapper: &MapLenToPathSet) {
    for len in b_mapper.keys() {
        if !a_mapper.contains_key(len) { continue }
        let a_paths: &PathSet = a_mapper.get(len).unwrap();
        let b_paths: &PathSet = b_mapper.get(len).unwrap();
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

pub fn detect_duplicates(a: &MapLenToPathSet, b: &MapLenToPathSet) {
    print_duplicates(a, b)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[1]);
    // Process the first directory path, which contains original files
    let path_a =  PathBuf::from(&args[1]);
    let path_a_mapper = dir_to_mapper(path_a);
    // Process the rest of the directory paths, which may contain clone files
    for arg in args[1..].iter() {
        let path_b =  PathBuf::from(arg);
        let path_b_mapper = dir_to_mapper(path_b);
        detect_duplicates(&path_a_mapper, &path_b_mapper)
    }
}

#[cfg(test)]
//#[macro_use] extern crate assert_matches;
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[test]
    /// Test `vet_path_is_dir` with valid.
    /// Must be true.
    /// 
    fn test_vet_path_is_dir_x_valid() {
        // Any dir is valid
        let path: PathBuf = [env!("CARGO_MANIFEST_DIR")].iter().collect();
        assert_eq!(
            vet_path_is_dir(path).unwrap(), 
            true,
        );
    }

    #[test]
    /// Test `vet_path_is_dir` with invalid.
    /// Must err.
    /// 
    fn test_vet_path_is_dir_x_invalid() {
        // Any file is invalid
        let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "vet_path", "file.txt"].iter().collect();
        assert_eq!(
            vet_path_is_dir(path).unwrap_err().kind(),
            io::ErrorKind::InvalidInput,
        );
    }

    #[test]
    /// Test `vet_path_is_file` with valid input.
    /// Must be true.
    /// 
    fn test_vet_path_is_file_x_valid() {
        // Any file is valid
        let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "vet_path", "file.txt"].iter().collect();
        assert_eq!(
            vet_path_is_file(path).unwrap(), 
            true
        );
    }

    #[test]
    /// Test `vet_path_is_file` with invalid.
    /// Must err.
    /// 
    fn test_vet_path_is_file_x_invalid() {
        // Any dir is invalid
        let path: PathBuf = [env!("CARGO_MANIFEST_DIR")].iter().collect();
        assert_eq!(
            vet_path_is_file(path).unwrap_err().kind(), 
            io::ErrorKind::InvalidInput,
        ); 
    }

    #[test]
    /// Test `file_len_eq` with equal files.
    /// Must be true.
    /// 
    fn test_file_len_eq_x_equal() {
        let a: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "file_len_eq", "alpha.txt"].iter().collect();
        let b: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "file_len_eq", "bravo.txt"].iter().collect();
        assert_eq!(
            file_len_eq(a, b).unwrap(), 
            true,
        );
    }

    #[test]
    /// Test `file_len_eq` with inequal files.
    /// Must be false.
    ///
    fn test_file_len_eq_x_inequal() {
        let a: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "file_len_eq", "alpha.txt"].iter().collect();
        let b: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "file_len_eq", "charlie.txt"].iter().collect();
        assert_eq!(
            file_len_eq(a, b).unwrap(), 
            false,
        );
    }

    #[test]
    /// Test `file_len_eq` with invalid args.
    /// Must err.
    ///
    fn test_file_len_eq_x_invalid_args() {
        let valid_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "file_len_eq", "alpha.txt"].iter().collect();
        let invalid_path: PathBuf = [env!("CARGO_MANIFEST_DIR")].iter().collect(); // i.e. anything that's not a file
        assert_eq!(
            file_len_eq(valid_path.clone(), invalid_path.clone()).unwrap_err().kind(), 
            io::ErrorKind::InvalidInput,
        );
        assert_eq!(
            file_len_eq(invalid_path.clone(), valid_path.clone()).unwrap_err().kind(), 
            io::ErrorKind::InvalidInput,
        );
    }

    #[test]
    /// Test `file_bytes_eq` with equal files.
    /// Must be true.
    ///  
    fn test_file_bytes_eq_x_equal() {
        let a: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "file_bytes_eq", "alpha.txt"].iter().collect();
        let b: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "file_bytes_eq", "alpha_2.txt"].iter().collect();
        assert_eq!(file_bytes_eq(a, b).unwrap(), true);
    }

    #[test]
    /// Test `file_bytes_eq` with inequal files. 
    /// Must be false.
    ///
    fn test_file_bytes_eq_x_inequal() {
        let a: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "file_bytes_eq", "alpha.txt"].iter().collect();
        let b: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "file_bytes_eq", "bravo.txt"].iter().collect();
        assert_eq!(file_bytes_eq(a, b).unwrap(), false);
    }
 
    #[test]
    /// Test `dir_to_mapper` via these files:
    /// 
    /// * `alpha.txt` which contains `alpha` (len 6 on disk)
    /// * `bravo.txt` which contains `bravo` (len 6 on disk)
    /// * `charlie.txt` which contains `charlie` (len 8 on disk)
    /// 
    fn test_dir_to_mapper() {
        let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "dir_to_mapper"].iter().collect();
        let mapper = dir_to_mapper(path);
        assert_eq!(mapper.len(), 2, "len");
        assert!(mapper.contains_key(&6), "contains key 6 for the file `alpha.txt` and `bravo.txt`");
        assert!(mapper.contains_key(&8), "contains key 8 for the file `charlie.txt`");
        assert_eq!(mapper.get(&6).unwrap().len(), 2, "mapper.get(6) len");
        assert_eq!(mapper.get(&8).unwrap().len(), 1, "mapper.get(8) len");
    }

    #[test]
    /// Test `print_duplicates` via these files:
    /// 
    /// * `a/alpha.txt` which contains `alpha`
    /// * `a/bravo.txt` which contains `bravo`
    /// * `b/alpha.txt` which contains `alpha`
    /// * `b/bravo.txt` which contains `bravo`
    /// 
    fn test_print_duplicates() {
        let a_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "print_duplicates", "a"].iter().collect();
        let b_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "print_duplicates", "b"].iter().collect();
        let a_mapper = dir_to_mapper(a_path);
        let b_mapper = dir_to_mapper(b_path);
        detect_duplicates(&a_mapper, &b_mapper);
    }

}
