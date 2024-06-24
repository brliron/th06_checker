use th06_checker::file_hash::FileError;

#[cfg(windows)]
fn pause() {
    use std::io;

    println!("Press Enter to continue...");
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
}
// Unix programs don't open a terminal when double-clicking
// on a console program, so we're either run from a terminal,
// where this pause is useless, or from a GUI, where we
// can't interact with the user.
#[cfg(unix)]
fn pause() {
}

fn main() {
    let result = match th06_checker::check_th06_folder() {
        Ok(result) => result,
        Err(e) => {
            match e {
                FileError { filename, err } => println!("{filename}: {err}"),
            }
            return ()
        },
    };
    result.print();
    println!();
    if result.is_ok() {
        println!("This version seems fine.");
    } else {
        result.try_to_fix();
    }

    pause();
}
