use std::borrow::Cow;
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

// I don't think we ever had anyone on Linux with these
// kind of errors, so we don't bother with them.
// We could implement them properly if there ever is a need for them.
#[cfg(unix)]
#[allow(non_snake_case)]
unsafe fn GetACP() -> u32 {
    1252
}
#[cfg(unix)]
#[allow(non_snake_case)]
unsafe fn GetOEMCP() -> u32 {
    850
}

fn string_to_sjis_bytes(string: &str) -> Cow<'_, [u8]> {
    let (bytes, _encoding_used, had_errors) = SHIFT_JIS.encode(string);
    assert!(!had_errors, "{} contains characters invalid for shift-jis", string);
    bytes
}

fn get_encoding<F, G, T>(kind: &str, get_cp_from_os: F, cp_to_encoding: G) -> Option<T>
where
    F: FnOnce() -> u32,
    G: FnOnce(u16) -> Option<T>,
{
    let cp = get_cp_from_os();
    let cp: u16 = cp.try_into().expect("Code page bigger than 65536");
    let encoding = match cp_to_encoding(cp) {
        Some(encoding) => encoding,
        None => {
            println!("    Can't convert from {} encoding {}.", kind, cp);
            return None;
        }
    };
    Some(encoding)
}

fn convert_string_to_messed_ansi(s_in: &str) -> Option<String> {
    let sjis = string_to_sjis_bytes(s_in);
    let encoding = get_encoding("ANSI", || unsafe { GetACP() }, |cp| codepage::to_encoding(cp))?;
    let (s_out, _encoding_used, _had_errors) = encoding.decode(&sjis);

    Some(String::from(s_out))
}

fn convert_string_to_messed_oem(s_in: &str) -> Option<String> {
    let sjis = string_to_sjis_bytes(s_in);
    let encoding = get_encoding("OEM", || unsafe { GetOEMCP() }, |cp| DECODING_TABLE_CP_MAP.get(&cp))?;
    let s_out = encoding.decode_string_lossy(sjis);

    Some(s_out)
}

fn remove_nonprintable(filename: &str) -> String {
    return filename.chars().map(|x| if x.is_control() { '.' } else { x }).collect::<String>()
}

fn try_rename(src: Option<String>, dst: &str) -> bool {
    let src = match src {
        Some(s) => s,
        None => return false,
    };
    print!("    {}... ", remove_nonprintable(&src));
    if !Path::new(&src).is_file() {
        println!("{}", "not found".red());
        return false;
    }
    println!("{}, renaming to {}...", "found".green(), dst);

    match fs::rename(&src, &dst) {
        Ok(()) => { println!("    {}", "Success".green()); true },
        Err(e) => { println!("    {}: {}", "Error".red(), e); false },
    }
}

fn fix_filename(filename: &str) -> bool {
    print!("  {}... ", filename);

    if Path::new(filename).is_file() {
        println!("{}", "ok".green());
        return false;
    }
    println!("{}, trying to fix files with broken encodings...", "not found".yellow());

    try_rename(convert_string_to_messed_oem(filename), filename)
        || try_rename(convert_string_to_messed_ansi(filename), filename)
}

pub fn fix_all_filenames() -> bool {
    println!("Detecting files with encoding errors...");

    let mut fix_done = false;
    for f in [
        "東方紅魔郷.exe",
        "紅魔郷CM.DAT",
        "紅魔郷ED.DAT",
        "紅魔郷IN.DAT",
        "紅魔郷MD.DAT",
        "紅魔郷ST.DAT",
        "紅魔郷TL.DAT",
    ] {
        if fix_filename(&f) {
            fix_done = true;
        }
    }

    fix_done
}
