
use std::io::{stdout,Write,Read};
use std::fs::{OpenOptions};
use std::fmt::Display;

use super::parser_output::{AbstractSyntaxTree};

pub fn formatter(path: &str) -> Result<(),String> {
    let mut input_file = match OpenOptions::new().read(true).open(path) {
        Ok(input_file) => input_file,
        Err(e) => return Err(format!("failed to open file={:?} with error={:?}",path, e))
    };
    let mut file_data = String::with_capacity(4096);
    match input_file.read_to_string(&mut file_data) {
        Ok(_) => { },
        Err(e) => return Err(format!("failed to read file={:?} with error={:?}", path, e))
    };

    let tree = match AbstractSyntaxTree::parse(&file_data) {
        Ok(tree) => tree,
        Err(e) => return Err(e),
    };
    println!("{}", tree);
    Ok(())
}

