use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::PathBuf;

use colored::*;
use ini::Ini;

#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{ GetSystemMetrics, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN };

use crate::file_hash::FileHash;
use crate::file_error::FileError;

#[cfg(windows)]
#[allow(non_snake_case)]
fn get_screen_size() -> (u32, u32) {
    unsafe {
        let width  = u32::try_from(GetSystemMetrics(SM_CXVIRTUALSCREEN)).unwrap();
        let height = u32::try_from(GetSystemMetrics(SM_CYVIRTUALSCREEN)).unwrap();
        (width, height)
    }
}
#[cfg(unix)]
fn get_screen_size() -> (u32, u32) {
    // TODO
    (1920, 1080)
}

enum FileStatus {
    Good,
    NotFound,
    WrongHash,
}

pub struct VpatchConfig {
    is_enabled: Option<bool>, // If present, should be 1
    width: Option<u32>, // If present, should match screen
    height: Option<u32>, // If present, should match screen
}

impl VpatchConfig {
    fn unwrap_u32(x: Option<&str>, filename: &'static str) -> Result<Option<u32>, FileError> {
        match x {
            Some(x) => match x.parse() {
                Ok(x) => Ok(Some(x)),
                Err(_) => Err(FileError::new_format(
                    format!("Failed to parse {} as integer", x),
                    filename)),
            },
            None => Ok(None),
        }
    }

    pub fn load_from_file(filename: &'static str) -> Result<Option<Ini>, FileError> {
        match Ini::load_from_file(filename) {
            Ok(x) => Ok(Some(x)),
            Err(e) => match e {
                ini::Error::Io(e) => match e.kind() {
                    io::ErrorKind::NotFound => Ok(None),
                    _ => Err(FileError::from_io(e, filename)),
                },
                ini::Error::Parse(e) => Err(FileError::from_parse(e, filename)),
            }
        }
    }

    pub fn parse(filename: &'static str) -> Result<Option<VpatchConfig>, FileError> {
        let conf = match Self::load_from_file(filename)? {
            Some(conf) => conf,
            None => return Ok(None),
        };
        let section_vpatch = match conf.section(Some("Window")) {
            Some(x) => x,
            None => return Ok(Some(VpatchConfig {
                is_enabled: None,
                width: None,
                height: None,
            })),
        };
        let is_enabled = match section_vpatch.get("enabled"){
            Some(x) => match x {
                "1" => Some(true),
                "0" => Some(false),
                x   => return Err(FileError::new_format(
                    format!("Failed to parse {} as either 1 or 0", x),
                    filename)),
            },
            None => None,
        };
        let width = Self::unwrap_u32(section_vpatch.get("Width"), filename)?;
        let height = Self::unwrap_u32(section_vpatch.get("Height"), filename)?;
        Ok(Some(VpatchConfig {
            is_enabled,
            width,
            height,
        }))
    }

    pub fn is_width_good(&self) -> bool {
        match self.width {
            Some(x) => x <= get_screen_size().0,
            None => true,
        }
    }
    pub fn is_height_good(&self) -> bool {
        match self.height {
            Some(x) => x <= get_screen_size().1,
            None => true,
        }
    }
    pub fn is_good(&self) -> bool {
        if let Some(x) = self.is_enabled {
            if !x {
                return false;
            }
        }
        return self.is_width_good() &&
            self.is_height_good();
    }
    pub fn is_option_good(o: &Option<Self>) -> bool {
        match o {
            Some(x) => x.is_good(),
            None    => false,
        }
    }

    pub fn to_string(&self) -> String {
        let is_enabled = match self.is_enabled {
            Some(x) => match x {
                true => "true".green(),
                false => "false".green(),
            }
            None => "not filled".yellow(),
        };
        let width = match self.width {
            Some(x) => {
                let s = x.to_string();
                if self.is_width_good() {
                    s.green()
                } else {
                    s.red()
                }
            }
            None => "not filled".yellow(),
        };
        let height = match self.height {
            Some(x) => {
                let s = x.to_string();
                if self.is_height_good() {
                    s.green()
                } else {
                    s.red()
                }
            }
            None => "not filled".yellow(),
        };
        format!(r"{}
    enabled: {}
    width: {}
    height: {}",
            "present".green().to_string(),
            is_enabled.to_string(),
            width.to_string(),
            height.to_string(),
        )
    }
    pub fn option_to_string(o: &Option<Self>) -> String {
        match o {
            Some(x) => x.to_string(),
            None    => "missing".red().to_string(),
        }
    }
}

pub struct Vpatch {
    main_executable: FileStatus,
    dll: FileStatus,
    // ini: bool,
    ini: Option<VpatchConfig>,
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
        for path in FileError::convert_io(fs::read_dir("."), ".")? {
            let path = FileError::convert_io(path, ".")?.path();
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
        VpatchConfig::is_option_good(&self.ini) &&
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
            VpatchConfig::option_to_string(&self.ini),
            self.other_dlls_to_string(),
        )
    }

    pub fn check() -> Result<Vpatch, FileError> {
        Ok(Vpatch {
            main_executable: Self::check_file("vpatch.exe", "29a933678de5dc4bf7941ff8587e3fe2a4794f3cfdad94453200151376f6388a")?,
            dll: Self::check_file("vpatch_th06_unicode.dll", "cc2513317da9ea8c832ef6d9cd95d12ead14b991a1eaed2d4c0fc27978b74e04")?,
            ini: VpatchConfig::parse("vpatch.ini")?, // Path::new("vpatch.ini").is_file(),
            other_dlls: Self::check_other_dlls()?,
        })
    }
}
