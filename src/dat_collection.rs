use colored::*;

use crate::file_hash::{FileHash, FileError};

pub struct Dat {
    file: FileHash,
    expected_hash: &'static str,
}

impl Dat {
    fn create(filename: &'static str, expected_hash: &'static str) -> Result<Self, FileError> {
        Ok(Dat {
            file: FileHash::read(filename)?,
            expected_hash
        })
    }

    fn is_valid(&self) -> bool {
        match &self.file {
            FileHash::File { hash, .. } => hash == self.expected_hash,
            _ => false,
        }
    }

    fn is_present(&self) -> bool {
        match &self.file {
            FileHash::File { .. } => true,
            _ => false,
        }
    }

    fn to_string_expect_valid(&self) -> String {
        match &self.file {
            FileHash::File { filename, .. } => format!("  {}: {}", filename,
                if self.is_valid() {
                    "ok".green()
                } else {
                    "invalid hash".red()
                }),
            FileHash::NotFound(filename) => format!("  {}: {}", filename, "not found".red()),
        }
    }

    fn to_string_expect_missing(&self) -> String {
        match &self.file {
            FileHash::NotFound(filename) => format!("  {}: {}", filename, "not found".green()),
            FileHash::File { filename, .. } => format!("  {}: {}", filename, "present".yellow()),
        }
    }
}

pub struct DatCollection {
    dat_cm: Dat,
    dat_ed: Dat,
    dat_in: Dat,
    dat_md: Dat,
    dat_st: Dat,
    dat_tl: Dat,
}

impl DatCollection {
    pub fn create_jp() -> Result<Self, FileError> {
        Ok(DatCollection {
            dat_cm: Dat::create("紅魔郷CM.DAT", "a899853d04e214ae4df8090bad7fd42698527027aa9dfccb4650fbb1d7828a0a")?,
            dat_ed: Dat::create("紅魔郷ED.DAT", "3fbb51f00785c98d6b4141a7a5a303f5955df3d181d2f220c2c6e81d717e9fee")?,
            dat_in: Dat::create("紅魔郷IN.DAT", "65d7ee9c4303bcb39f5f08a0ceaf7004e47fccc8242fd73db54b31a911f41af0")?,
            dat_md: Dat::create("紅魔郷MD.DAT", "8f8db1918842857a63eb7c76e7f971fb931203a6239c26828304fa3ce12da911")?,
            dat_st: Dat::create("紅魔郷ST.DAT", "0f834a35aef2d73b05cffecc830c017dacbcc6f11b9a0611a9da2f3970a112e7")?,
            dat_tl: Dat::create("紅魔郷TL.DAT", "c05f4fa755602f9369d7cebd5689cf3655ec81bb746f5b269ee0faf3d5f0a020")?,
        })
    }

    pub fn create_en() -> Result<Self, FileError> {
        Ok(DatCollection {
            // We won't check these hashes, no need to fill them
            dat_cm: Dat::create("th06e_CM.DAT", "")?,
            dat_ed: Dat::create("th06e_ED.DAT", "")?,
            dat_in: Dat::create("th06e_IN.DAT", "")?,
            dat_md: Dat::create("th06e_MD.DAT", "")?,
            dat_st: Dat::create("th06e_ST.DAT", "")?,
            dat_tl: Dat::create("th06e_TL.DAT", "")?,
        })
    }

    pub fn is_valid(&self) -> bool {
        self.iter().all(|x| x.is_valid())
    }

    pub fn is_present(&self) -> bool {
        self.iter().any(|x| x.is_present())
    }

    pub fn to_string_expect_valid(&self) -> String {
        self.iter().fold(String::new(), |mut acc, x| {
            if acc != "" {
                acc += "\n";
            }
            acc + &x.to_string_expect_valid()
        })
    }

    pub fn to_string_expect_missing(&self) -> String {
        self.iter().fold(String::new(), |mut acc, x| {
            if acc != "" {
                acc += "\n";
            }
            acc + &x.to_string_expect_missing()
        })
    }

    fn iter(&self) -> Iter {
        Iter {
            obj: self,
            curr: 0,
        }
    }
}

struct Iter<'a> {
    obj: &'a DatCollection,
    curr: u8,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Dat;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.curr {
            0 => Some(&self.obj.dat_cm),
            1 => Some(&self.obj.dat_ed),
            2 => Some(&self.obj.dat_in),
            3 => Some(&self.obj.dat_md),
            4 => Some(&self.obj.dat_st),
            5 => Some(&self.obj.dat_tl),
            _ => None,
        };
        self.curr += 1;
        next
    }
}
