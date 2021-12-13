use std::{fs,io};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// File buffer size fpr comparing bytes
const FILE_BUFFER_SIZE: usize = 8192;

/// Do two files have equal length?
pub fn assure_paths_metadata_len_eq<T: AsRef<Path>>(a: T, b: T) -> io::Result<bool> {
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

/// Do two files have equal bytes?
pub fn assure_paths_read_bytes_eq<T: AsRef<Path>>(a: T, b: T) -> io::Result<bool> {
    let mut a_file = File::open(a)?;
    let mut b_file = File::open(b)?;
    let mut a_buffer = [0; FILE_BUFFER_SIZE];
    let mut b_buffer = [0; FILE_BUFFER_SIZE];
    loop {
        let a_n = a_file.read(&mut a_buffer)?;
        let b_n = b_file.read(&mut b_buffer)?;
        if a_n == 0 && b_n == 0 { return Ok(true) }
        if a_n != b_n { return Ok(false) }
        if a_buffer[0..a_n] != b_buffer[0..b_n] { return Ok(false) }
    }
}

#[cfg(test)]
//#[macro_use] extern crate assert_matches;
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[test]
    /// Test `file_len_eq` with equal files.
    /// Must be true.
    /// 
    fn test_file_len_eq_x_equal() {
        let a: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "file_len_eq", "alpha.txt"].iter().collect();
        let b: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "file_len_eq", "bravo.txt"].iter().collect();
        assert_eq!(file_len_eq(a, b).unwrap(), true);
    }

    #[test]
    /// Test `file_len_eq` with inequal files.
    /// Must be false.
    ///
    fn test_file_len_eq_x_inequal() {
        let a: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "file_len_eq", "alpha.txt"].iter().collect();
        let b: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "file_len_eq", "charlie.txt"].iter().collect();
        assert_eq!(file_len_eq(a, b).unwrap(), false);
    }

    #[test]
    /// Test `file_len_eq` with invalid args.
    /// Must err.
    ///
    fn test_file_len_eq_x_invalid_args() {
        let valid: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "file_len_eq", "alpha.txt"].iter().collect();
        let invalid: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "file_len_eq"].iter().collect(); // i.e. anything that's not a file
        assert_eq!(file_len_eq(valid.clone(), invalid.clone()).unwrap_err().kind(), io::ErrorKind::InvalidInput);
        assert_eq!(file_len_eq(invalid.clone(), valid.clone()).unwrap_err().kind(), io::ErrorKind::InvalidInput);
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

}
