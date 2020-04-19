use std::fs::OpenOptions;

pub fn file_path(arg: String) -> Result<(), String> {
    match OpenOptions::new().read(true).open(&arg) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("failed to open file: {} with error {:?}", &arg, e)),
    }
}
