pub mod file_hash;
mod messed_encoding;
mod version;
mod dat_collection;

use std::path::Path;
use colored::*;

use file_hash::FileError;
use version::MainExecutableStatus;
use dat_collection::DatCollection;

pub struct Th06Result {
    pub main_executable: MainExecutableStatus,
    pub has_eosd_exe: bool,
    pub has_th06e_exe: bool,
    pub dat_jp: DatCollection,
    pub dat_en: DatCollection,
}

impl Th06Result {
    pub fn is_ok(&self) -> bool {
        self.main_executable.is_good() &&
            !self.has_eosd_exe &&
            !self.has_th06e_exe
    }

    fn yn(b: bool, colors: &str) -> ColoredString {
        assert!(colors.len() == 2);
        let (s, color) = if b {
            ("yes", colors.chars().nth(0).unwrap())
        } else {
            ("no",  colors.chars().nth(1).unwrap())
        };
        match color {
            'g' => s.green(),
            'y' => s.yellow(),
            'r' => s.red(),
            _ => panic!(),
        }
    }

    pub fn print(&self) {
        println!("東方紅魔郷.exe status: {}", self.main_executable.to_string());
        println!("eosd.exe exists? {}",  Self::yn(self.has_eosd_exe, "rg"));
        println!("th06e.exe exists? {}", Self::yn(self.has_th06e_exe, "rg"));
        println!("Original dat files are present? {}", Self::yn(self.dat_jp.is_valid(), "gr"));
        println!("{}", self.dat_jp.to_string_expect_valid());
        println!("English dat files are present? {}", Self::yn(self.dat_en.is_present(), "yg"));
        println!("{}", self.dat_en.to_string_expect_missing());
    }
}

pub fn check_th06_folder() -> Result<Th06Result, FileError> {
    messed_encoding::fix_all_filenames();
    Ok(Th06Result {
        main_executable: version::identify()?,
        has_eosd_exe:    Path::new("eosd.exe").is_file(),
        has_th06e_exe:   Path::new("th06e.exe").is_file(),
        dat_jp:          DatCollection::create_jp()?,
        dat_en:          DatCollection::create_en()?,
    })
}
