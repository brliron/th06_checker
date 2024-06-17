use colored::*;

use crate::file_hash::{FileHash, FileError};

pub struct Version {
    pub name: &'static str,
    pub hash: &'static str,
    pub is_good: bool,
}

pub enum MainExecutableStatus {
    Version(&'static Version),
    UnknownVersion(String),
    Missing,
}

impl MainExecutableStatus {
    pub fn is_good(&self) -> bool {
        match self {
            MainExecutableStatus::Version(v) => v.is_good,
            _ => false,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            MainExecutableStatus::Version(v) => format!(
                "{} - {}", v.name, if v.is_good { "good".green() } else { "incorrect".red() }
            ),
            MainExecutableStatus::UnknownVersion(hash) => format!("unknown version ({})", hash).red().to_string(),
            MainExecutableStatus::Missing => "not found".red().to_string(),
        }
    }
}

fn hash_to_version(hash: &str) -> Option<&'static Version> {
    let versions: [&'static Version; 9] = [
        &Version { name: "v0.08 (original)",  hash: "1eb4ab98622e08e24247cbdd692671d7abca840de3239f147a4f7c377a175552", is_good: false, },
        &Version { name: "v0.13 (original)",  hash: "21cddb2b5a7dfaf38e542b707873a35b3fbfd38c1aa21d5b969e1a49a1855f4b", is_good: false, },
        &Version { name: "v1.00 (original)",  hash: "7152c0ce2667ff10bd5bbe9c3a2052302dec7d326798eb2e70f106e933831a9d", is_good: false, },
        &Version { name: "v1.02f (original)", hash: "0adcf7ad5b451d77b5fb86771c3718d242fc7b27dc5940477d289a797e51785f", is_good: false, },
        &Version { name: "v1.02h (original)", hash: "9f76483c46256804792399296619c1274363c31cd8f1775fafb55106fb852245", is_good: true, },
        &Version { name: "v1.02h original with 紅魔郷 removed from the .dat file names", hash: "7f38496b31b8625196900a69cd1bfed243cab1f9612073e7881dc354b876fd39", is_good: false, },
        &Version { name: "v1.02h English patch", hash: "0883c6433b77af87054d9d05731649c79926a55037840c72d33e50635d64d506", is_good: false, },
        &Version { name: "v1.02h English patch with a leftover .cfg removed", hash: "fa6562ddfd81f3010d7d87792a69aaa950e0f60b00e42885ef2c30577d8dbe45", is_good: false, },
        &Version { name: "v1.02h Russian patch", hash: "8a509709650a83db6850c3498e1e1051dddc6cd46791911c0d78438be8968195", is_good: false, },
    ];

    versions.iter().find(|x| x.hash == hash).copied()
}

pub fn identify() -> Result<MainExecutableStatus, FileError> {
    let exe_fn = "東方紅魔郷.exe";

    let hash = match FileHash::read(exe_fn)? {
        FileHash::File { hash, .. } => hash,
        FileHash::NotFound(_) => return Ok(MainExecutableStatus::Missing),
    };
    match hash_to_version(&hash) {
        Some(version) => Ok(MainExecutableStatus::Version(version)),
        None => Ok(MainExecutableStatus::UnknownVersion(hash)),
    }
}
