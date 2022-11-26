use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file = &args[2];

    let config = GrepConfig {
        query: query.clone(),
        file: file.clone(),
    };

    match fs::read_to_string(config.file) {
        Ok(contents) => {
            println!("With text:\n{}", contents);
        }
        Err(error) => {
            println!("Problem reading the file: {:?}", error);
        }
    }
    dbg!(args);
}

struct GrepConfig {
    query: String,
    file: String,
}
