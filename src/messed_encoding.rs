use std::fs;
use std::path::Path;
use colored::*;
use encoding_rs::*;
use codepage;

#[cfg(windows)]
extern "C" {
    fn GetACP() -> u32;
}
#[cfg(unix)]
#[allow(non_snake_case)]
fn GetACP() -> u32 {
    1252
}

fn convert_string_to_messed(s_in: &str) -> String {
    let (cow, _encoding_used, had_errors) = SHIFT_JIS.encode(s_in);
    assert!(!had_errors, "{} contains characters invalid for shift-jis", s_in);

    let cp = unsafe { GetACP() };
    let cp: u16 = cp.try_into().expect("Code page bigger than 65536");
    let encoding = codepage::to_encoding(cp).unwrap(); // TODO handle
    let (s_out, _encoding_used, _had_errors) = encoding.decode(&cow);

    String::from(s_out)
}

fn fix_filename(filename: &str) {
    print!("  {}... ", filename);

    if Path::new(filename).is_file() {
        println!("{}", "ok".green());
        return;
    }
    println!("{}, trying to fix files with broken encodings...", "not found".yellow());

    let filename_messed = convert_string_to_messed(filename);
    print!("    {}... ", filename_messed);
    if !Path::new(&filename_messed).is_file() {
        println!("{}", "not found".red());
        return;
    }
    println!("{}, renaming to {}...", "found".green(), filename);

    match fs::rename(&filename_messed, &filename) {
        Ok(()) => println!("    {}", "Success".green()),
        Err(e) => println!(    "{}: {}", "Error".red(), e),
    }
}

pub fn fix_all_filenames() {
    println!("Detecting encoding errors...");

    for f in [
        "東方紅魔郷.exe",
        "紅魔郷CM.DAT",
        "紅魔郷CM.DAT",
        "紅魔郷CM.DAT",
        "紅魔郷CM.DAT",
        "紅魔郷CM.DAT",
        "紅魔郷CM.DAT",
    ] {
        fix_filename(&f);
    }
}
