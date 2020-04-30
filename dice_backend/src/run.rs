
use std::io::{Read, Write};
use std::fs::OpenOptions;
use super::runtime::{create_report};

pub fn run_path(path: &str) -> Result<String,String> {

    let mut input_file = match OpenOptions::new().read(true).open(path) {
        Ok(input_file) => input_file,
        Err(e) => return Err(format!("failed to open file={:?} with error={:?}",path, e))
    };
    let mut file_data = String::with_capacity(4096);
    match input_file.read_to_string(&mut file_data) {
        Ok(_) => { },
        Err(e) => return Err(format!("failed to read file={:?} with error={:?}", path, e))
    };

    match create_report(&file_data) {
        Ok(report) => Ok(report.serialize_report(None)),
        Err(e) => Err(e),
    }
}
