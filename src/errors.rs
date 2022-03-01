use std::fs::File;
use std::io::Error as IOError;

pub fn try_error() {
    match try_open_file("test.txt") {
        Ok(_) => {
            println!("Success!");
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

pub fn try_error_panic() {
    match try_open_file("test.txt") {
        Ok(_) => {
            println!("Success!");
        }
        Err(err) => {
            panic!("{}", err);
        }
    }
}

pub fn try_open_file(path: &str) -> Result<File, IOError> {
    let file = File::open(path)?;
    Ok(file)
}
