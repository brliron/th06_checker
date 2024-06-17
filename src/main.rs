use th06_checker::file_hash::FileError;

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
    if result.is_ok() {
        println!("No problems detected!");
    }
}
