use std::fs;

use crate::file_hash::FileError;

pub fn check() -> Result<bool, FileError> {
    Ok(!FileError::convert(
            fs::metadata("."), "[current directory]"
        )?.permissions().readonly())
}
