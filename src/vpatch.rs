use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use colored::*;

use crate::file_hash::{FileHash, FileError};

enum FileStatus {
    Good,
    NotFound,
    WrongHash,
}

pub struct Vpatch {
    main_executable: FileStatus,
    dll: FileStatus,
    ini: bool,
    other_dlls: Vec<String>,
}

impl Vpatch {
    fn check_file(filename: &'static str, expected_hash: &'static str) -> Result<FileStatus, FileError> {
        let hash = match FileHash::read(filename)? {
            FileHash::File { hash, .. } => hash,
            FileHash::NotFound(_) => return Ok(FileStatus::NotFound),
        };

        if hash == expected_hash {
            Ok(FileStatus::Good)
        } else {
            Ok(FileStatus::WrongHash)
        }
    }

    fn extract_path_components(path: &PathBuf) -> Option<(&str, OsString)> {
        Some((path.file_name()?.to_str()?, path.extension()?.to_ascii_lowercase()))
    }

    fn check_other_dlls() -> Result<Vec<String>, FileError> {
        let mut vec = Vec::new();
        for path in FileError::convert(fs::read_dir("."), ".")? {
            let path = FileError::convert(path, ".")?.path();
            if let Some((filename, extension)) = Self::extract_path_components(&path) {
                if filename.starts_with("vpatch") && extension == "dll"
                    && filename != "vpatch_th06_unicode.dll" {
                    vec.push(String::from(filename));
                }
            }
        }
        Ok(vec)
    }

    pub fn is_good(&self) -> bool {
        matches!(self.main_executable, FileStatus::Good) &&
        matches!(self.dll, FileStatus::Good) &&
        self.ini &&
        self.other_dlls.len() == 0
    }

    fn status_to_string(status: &FileStatus) -> ColoredString {
        match status {
            FileStatus::Good => "ok".green(),
            FileStatus::NotFound => "missing".red(),
            FileStatus::WrongHash => "wrond hash".red(),
        }
    }

    fn other_dlls_to_string(&self) -> ColoredString {
        if self.other_dlls.len() == 0 {
            "none".green()
        } else {
            self.other_dlls.iter().fold(String::new(), |acc, x| {
                acc + "\n    " + x
            }).red()
        }
    }

    pub fn to_string(&self) -> String {
        format!(
r"  vpatch.exe: {}
  vpatch_th06_unicode.dll: {}
  vpatch.ini: {}
  Other dlls: {}",
            Self::status_to_string(&self.main_executable),
            Self::status_to_string(&self.dll),
            if self.ini { "present".green() } else { "missing".red() },
            self.other_dlls_to_string(),
        )
    }

    pub fn check() -> Result<Vpatch, FileError> {
        Ok(Vpatch {
            main_executable: Self::check_file("vpatch.exe", "29a933678de5dc4bf7941ff8587e3fe2a4794f3cfdad94453200151376f6388a")?,
            dll: Self::check_file("vpatch_th06_unicode.dll", "cc2513317da9ea8c832ef6d9cd95d12ead14b991a1eaed2d4c0fc27978b74e04")?,
            ini: Path::new("vpatch.ini").is_file(),
            other_dlls: Self::check_other_dlls()?,
        })
    }
}
