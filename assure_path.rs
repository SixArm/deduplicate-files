use std::{io,fs};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path};


/// Assure a condition is true.
///
/// If true, then return Ok(true).
///
/// Otherwise, return Err(std::io::Error …).
///
/// This macro has a second form, where a custom
/// message can be provided.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate assure; fn main() {
/// assure_io!(true);
/// assure_io!(true, "message");
/// # }
/// ```
#[macro_export]
macro_rules! assure_path_is_dir {
    ($path:expr $(,)?) => ({
        // pub fn <T: AsRef<Path>>(path: T) -> io::Result<bool>
        assure_io!(std::fs::metadata($path)?.is_dir())
    });
    ($path:expr, $($arg:tt)+) => ({
        assure_io!(std::fs::metadata(path)?.is_dir(), $($arg)+)
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assure_path_is_dir_x_arity_1_return_ok() {
        let x = assure_io!(true);
        assert!(x.is_ok());
    } 

    #[test]
    fn test_assure_io_x_arity_3_return_ok() {
        let x = assure_io!(true, "message");
        assert!(x.is_ok());
    } 

    #[test]
    fn test_assure_io_x_arity_2_return_err() {
        let x = assure_io!(false);
        assert!(x.is_err());
    } 

    #[test]
    fn test_assure_io_x_arity_3_return_err() {
        let x = assure_io!(false, "message");
        assert!(x.is_err());
    } 

}



#[allow(dead_code)]

#[allow(dead_code)]
pub fn assure_path_is_file_eq<T: AsRef<Path>>(path: T, value: bool) -> io::Result<bool> {
    assure_io_eq!(fs::metadata(path)?.is_file(), value, "assure_path_is_file")
}

#[allow(dead_code)]
pub fn assure_path_permissions_readonly_eq<T: AsRef<Path>>(path: T, value: bool) -> io::Result<bool> {
    assure_io_eq!(fs::metadata(path)?.permissions().readonly(), value, "assure_path_permissions_readonly")
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
            assure_path_is_dir!(path).unwrap(), 
            true
        );
    }

    #[test]
    /// Test `assure_path_is_dir` with not-dir.
    /// Must err.
    /// 
    fn test_assure_path_is_dir_eq_x_not_dir() {
        let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "assure", "assure_path_is_dir_eq", "file.txt"].iter().collect(); // Any not-dir
        assert_eq!(
            assure_path_is_dir_eq(path, false).unwrap_err().kind(),
            std::io::ErrorKind::InvalidInput,
        );
    }

    #[test]
    /// Test `assure_path_is_file` with file.
    /// Must be true.
    /// 
    fn test_assure_path_is_file_x_file() {
        let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "assure", "assure_path_is_file_eq", "file.txt"].iter().collect(); // Any file
        assert_eq!(
            assure_path_is_file_(path).unwrap(),
            true
        );
    }

    #[test]
    /// Test `assure_path_is_file` with not-file.
    /// Must err.
    /// 
    fn test_assure_path_is_file_x_not_file() {
        let path: PathBuf = [env!("CARGO_MANIFEST_DIR")].iter().collect(); // Any not-file
        assert_eq!(
            assure_path_is_file(path, true).unwrap_err().kind(), 
            io::ErrorKind::InvalidInput
        ); 
    }

    #[test]
    /// Test `assure_path_permissions_readonly` with readonly.
    /// Must be true.
    /// 
    fn test_assure_path_permissions_readonly_eq_x_readonly_and_true() {
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
    fn test_assure_path_permissions_readonly_eq_x_not_readonly_and_true() {
        let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "assure", "assure_path_permissions_readonly_eq", "false.txt"].iter().collect();
        assert_eq!(
            assure_path_permissions_readonly(path, true).unwrap_err().kind(), 
            io::ErrorKind::InvalidInput
        ); 
    }

}
