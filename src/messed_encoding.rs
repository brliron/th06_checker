use std::fs;
use std::path::Path;
use colored::*;
use encoding_rs::*;
use oem_cp::code_table::DECODING_TABLE_CP_MAP;
use codepage;

#[cfg(windows)]
extern "C" {
    fn GetACP() -> u32;
    fn GetOEMCP() -> u32;
}
#[cfg(unix)]
#[allow(non_snake_case)]
fn GetACP() -> u32 {
    1252
}
#[cfg(unix)]
#[allow(non_snake_case)]
fn GetOEMCP() -> u32 {
    850
}

fn convert_string_to_messed_ansi(s_in: &str) -> String {
    // TODO put this small bit into a function to share it with the other function
    let (cow, _encoding_used, had_errors) = SHIFT_JIS.encode(s_in);
    assert!(!had_errors, "{} contains characters invalid for shift-jis", s_in);

    let cp = unsafe { GetACP() };
    let cp: u16 = cp.try_into().expect("Code page bigger than 65536");
    let encoding = codepage::to_encoding(cp).unwrap(); // TODO handle
    let (s_out, _encoding_used, _had_errors) = encoding.decode(&cow);

    String::from(s_out)
}

fn convert_string_to_messed_oem(s_in: &str) -> String {
    let (cow, _encoding_used, had_errors) = SHIFT_JIS.encode(s_in);
    assert!(!had_errors, "{} contains characters invalid for shift-jis", s_in);

    let cp = unsafe { GetOEMCP() };
    let cp: u16 = cp.try_into().expect("Code page bigger than 65536");
    let encoding = DECODING_TABLE_CP_MAP.get(&cp).unwrap(); // TODO handle
    let s_out = encoding.decode_string_lossy(cow);

    s_out
}

fn fix_filename(filename: &str) {
    print!("  {}... ", filename);

    if Path::new(filename).is_file() {
        println!("{}", "ok".green());
        return;
    }
    println!("{}, trying to fix files with broken encodings...", "not found".yellow());

    // TODO try both ACP and OEMCP
    let filename_messed = convert_string_to_messed_oem(filename);
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
        "紅魔郷ED.DAT",
        "紅魔郷IN.DAT",
        "紅魔郷MD.DAT",
        "紅魔郷ST.DAT",
        "紅魔郷TL.DAT",
    ] {
        fix_filename(&f);
    }
}
