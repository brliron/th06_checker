use std::fs;
use std::io;

use colored::*;

use crate::file_error::FileError;

fn fix_or_error() -> io::Result<()> {
    let mut perms = fs::metadata(".")?.permissions();
    perms.set_readonly(false);
    fs::set_permissions(".", perms)?;
    Ok(())
}

pub fn fix() -> bool {
    match fix_or_error() {
        Ok(()) => true,
        Err(e) => {
            println!("{}: {}", "Error".red(), e);
            false
        },
    }
}

pub fn check() -> Result<bool, FileError> {
    Ok(!FileError::convert_io(
            fs::metadata("."), "[current directory]"
        )?.permissions().readonly())
}
