use std::fs;

use crate::file_hash::FileError;

pub fn check() -> Result<bool, FileError> {
    match fs::metadata(".") {
        Ok(metadata) => Ok(!metadata.permissions().readonly()),
        Err(e) => Err(FileError {
            filename: "[current directory]",
            err: e,
        }),
    }
}
