use std::{io,fs};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path};

#[allow(dead_code)]
pub fn assure_path_is_dir<T: AsRef<Path>>(path: T, value: bool) -> io::Result<bool> {
    assure_io!(fs::metadata(path)?.is_dir())
}

#[allow(dead_code)]
pub fn assure_path_is_file_eq<T: AsRef<Path>>(path: T, value: bool) -> io::Result<bool> {
    assure_io!(fs::metadata(path)?.is_file())
}

#[allow(dead_code)]
pub fn assure_path_permissions_readonly_eq<T: AsRef<Path>>(path: T, value: bool) -> io::Result<bool> {
    assure_io!(fs::metadata(path)?.permissions().readonly())
}

#[allow(dead_code)]
pub fn assure_path_permissions_mode_eq<T: AsRef<Path>>(path: T, value: u32) -> io::Result<bool> {
    assure_io_eq!(fs::metadata(path)?.permissions().mode(), value, "assure_path_permissions_mode")
}

#[cfg(test)]
//#[macro_use] extern crate assert_matches;
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[test]
    /// Test `assure_path_is_dir` with dir.
    /// Must be true.
    /// 
    fn test_assure_path_is_dir_x_dir() {
        let path: PathBuf = [env!("CARGO_MANIFEST_DIR")].iter().collect(); // Any dir
        assert_eq!(
            assure_path_is_dir(path, true).unwrap(), 
            true,
        );
    }

    #[test]
    /// Test `assure_path_is_dir` with not-dir.
    /// Must err.
    /// 
    fn test_assure_path_is_dir_x_not_dir_and_true() {
        let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "assure", "assure_path_is_dir", "file.txt"].iter().collect(); // Any not-dir
        assert_eq!(
            assure_path_is_dir(path, true).unwrap_err().kind(),
            io::ErrorKind::InvalidInput,
        );
    }

    #[test]
    /// Test `assure_path_is_file` with file.
    /// Must be true.
    /// 
    fn test_assure_path_is_file_x_file_and_true() {
        let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "assure", "assure_path_is_file_eq", "file.txt"].iter().collect(); // Any file
        assert_eq!(
            assure_path_is_file(path, true).unwrap(), 
            true
        );
    }

    #[test]
    /// Test `assure_path_is_file_eq` with not-file.
    /// Must err.
    /// 
    fn test_assure_path_is_file_x_not_file() {
        let path: PathBuf = [env!("CARGO_MANIFEST_DIR")].iter().collect(); // Any not-file
        assert_eq!(
            assure_path_is_file(path, true).unwrap_err().kind(), 
            io::ErrorKind::InvalidInput,
        ); 
    }

    #[test]
    /// Test `assure_path_permissions_readonly` with readonly.
    /// Must be true.
    /// 
    fn test_assure_path_permissions_readonly_x_readonly() {
        let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "assure", "assure_path_permissions_readonly_eq", "true.txt"].iter().collect();
        assert_eq!(
            assure_path_permissions_readonly(path, true).unwrap(), 
            true
        );
    }

    #[test]
    /// Test `assure_path_permissions_readonly` with not-readonly.
    /// Must err.
    /// 
    fn test_assure_path_permissions_readonly_x_not_readonly() {
        let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "assure", "assure_path_permissions_readonly_eq", "false.txt"].iter().collect();
        assert_eq!(
            assure_path_permissions_readonly(path, true).unwrap_err().kind(), 
            io::ErrorKind::InvalidInput,
        ); 
    }

}
