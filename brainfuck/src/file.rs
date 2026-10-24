use std::env;
use std::fs;

pub fn read_file(filenumber: u32) -> String {
    let path = env::current_dir();
    match path {
        Ok(_) => {
            const DEFAULT_FILE_PATH: &str = "data\\";
            const DEFAULT_FILE_EXTENSION: &str = ".bf";

            return fs::read_to_string(format!("{DEFAULT_FILE_PATH}{filenumber}{DEFAULT_FILE_EXTENSION}")).expect("Error reading file {filenumber}");
        }
        Err(e) => {println!("Could not get current working directory. {e}"); return String::new();}
    }
}
