use std::fs::File;
use std::io::prelude::*;

pub fn read_input_file(file_name: &str) -> String {
    
    match File::open(file_name) {
        Err(err) => {
            error!("Err reading file: {}", err.to_string());
            return String::new();
        },
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => {
                    return contents;
                }
                Err(err) => {
                    error!("Err getting file contents: {}", err.to_string());
                    return String::from("");
                }
            }        
        }
    }
}

pub fn eval_digit(ch: u8) -> Option<usize> {
    let digits = "0123456789".as_bytes();
    let result = digits.iter().position(|&r| r == ch);
    result
}