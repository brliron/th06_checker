use std::fmt;
use std::io;

use ini;

pub enum ErrorKind {
    Io(io::Error),
    Parse(ini::ParseError),
    Format(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::Io(e) => write!(f, "{}", e),
            ErrorKind::Parse(e) => write!(f, "{}", e),
            ErrorKind::Format(e) => write!(f, "{}", e),
        }
    }
}

pub struct FileError {
    pub filename: &'static str,
    pub err: ErrorKind,
}

impl FileError {
    pub fn from_io(e: io::Error, filename: &'static str) -> FileError {
        FileError {
            filename,
            err: ErrorKind::Io(e),
        }
    }

    pub fn from_parse(e: ini::ParseError, filename: &'static str) -> FileError {
        FileError {
            filename,
            err: ErrorKind::Parse(e),
        }
    }

    pub fn new_format(e: String, filename: &'static str) -> FileError {
        FileError {
            filename,
            err: ErrorKind::Format(e),
        }
    }

    pub fn convert_io<T>(result: Result<T, io::Error>, filename: &'static str) -> Result<T, FileError> {
        match result {
            Ok(success) => Ok(success),
            Err(e) => Err(FileError { filename: filename, err: ErrorKind::Io(e) }),
        }
    }

    pub fn convert_ini<T>(result: Result<T, ini::Error>, filename: &'static str) -> Result<T, FileError> {
        match result {
            Ok(success) => Ok(success),
            Err(e) => match e {
                ini::Error::Io(e) => Err(FileError { filename: filename, err: ErrorKind::Io(e) }),
                ini::Error::Parse(e) => Err(FileError { filename: filename, err: ErrorKind::Parse(e) }),
            }
        }
    }
}
