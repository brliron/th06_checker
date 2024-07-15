use std::fs;

use crate::file_error::FileError;

pub fn check() -> Result<bool, FileError> {
    Ok(!FileError::convert_io(
            fs::metadata("."), "[current directory]"
        )?.permissions().readonly())
}
