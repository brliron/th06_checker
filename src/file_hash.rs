use std::fs;
use std::io::{self, ErrorKind};
use sha256;

pub struct FileError {
    pub filename: &'static str,
    pub err: io::Error,
}

impl FileError {
    pub fn convert<T>(result: Result<T, io::Error>, filename: &'static str) -> Result<T, FileError> {
        match result {
            Ok(success) => Ok(success),
            Err(e) => Err(FileError { filename: filename, err: e }),
        }
    }
}

pub enum FileHash {
    File {
        filename: &'static str,
        hash: String,
    },
    NotFound(&'static str),
}

impl FileHash {
    pub fn read(filename: &'static str)
    -> Result<Self, FileError> {
        match fs::read(filename) {
            Ok(file) => Ok(Self::File { filename, hash: sha256::digest(file) }),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => Ok(Self::NotFound(filename)),
                _ => Err(FileError { filename: filename, err: e }),
            }
        }
    }
}
