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

    pub fn is_ok(&self) -> bool {
        self.main_executable.is_good() &&
            self.dat_jp.is_valid()
    }

    pub fn print(&self) {
        println!("東方紅魔郷.exe status: {}", self.main_executable.to_string());
        println!("eosd.exe exists? {}",  Self::yn(self.has_eosd_exe, "yg"));
        println!("th06e.exe exists? {}", Self::yn(self.has_th06e_exe, "yg"));
        println!("Original dat files are present? {}", Self::yn(self.dat_jp.is_valid(), "gr"));
        println!("{}", self.dat_jp.to_string_expect_valid());
        println!("English dat files are present? {}", Self::yn(self.dat_en.is_present(), "yg"));
        println!("{}", self.dat_en.to_string_expect_missing());
    }

    pub fn try_to_fix(&self) {
        match self.main_executable {
            MainExecutableStatus::Version(v) => {
                if !v.is_good {
                    if v.name.starts_with("v1.02h") {
                        println!("東方紅魔郷.exe seems to be a patched version ({}). It might not work properly.", v.name);
                    } else {
                        println!("東方紅魔郷.exe seems to be outdated ({}). It will probably not work properly.", v.name);
                        println!("You should install the v1.02h from https://www16.big.or.jp/~zun/html/th06.html");
                    }
                }
            },
            MainExecutableStatus::UnknownVersion(_) => println!("Unknown version for 東方紅魔郷.exe. It might be an outdated version, in which case, you can try to update to v1.02h with the installer at https://www16.big.or.jp/~zun/html/th06.html , or a patched version that we don't know, in which case it might or might now work."),
            MainExecutableStatus::Missing => {
                println!("東方紅魔郷.exe not found. Trying to fix...");
                if messed_encoding::fix_all_filenames() {
                    println!("You can try running this tool again to see the new status after this fix.");
                    return;
                }
                println!("東方紅魔郷.exe not found. Thcrap will probably not work. Reinstall the game from the CD or download it from somewhere else.");
            },
        }

        if !self.dat_jp.is_present() {
            println!("At lease one original dat file wasn't found. Trying to fix...");
            if messed_encoding::fix_all_filenames() {
                println!("You can try running this tool again to see the new status after this fix.");
                return;
            }
            println!("Fix failed. Thcrap will probably not work. Reinstall the game from the CD or download it from somewhere else.");
        }
    }
}

pub fn check_th06_folder() -> Result<Th06Result, FileError> {
    Ok(Th06Result {
        main_executable: version::identify()?,
        has_eosd_exe:    Path::new("eosd.exe").is_file(),
        has_th06e_exe:   Path::new("th06e.exe").is_file(),
        dat_jp:          DatCollection::create_jp()?,
        dat_en:          DatCollection::create_en()?,
    })
}
