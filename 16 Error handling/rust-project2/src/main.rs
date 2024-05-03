use core::fmt;
use std::{fmt::{Debug, Formatter}, fs};

pub struct FileReadError {

}

fn main() {
    let contents = read_file("hello.txt".to_string());
    match contents {
        Ok(file_content) => {
            println!("File content: {}", file_content);
        },
        Err(error) => {
            println!("Error reading file: {:?}", error);
        }
    }
}

fn read_file(file_path: String) -> Result<String, FileReadError> {
    let greeting_file_result = fs::read_to_string("hello.txt");
    match greeting_file_result {
        Ok(file_content) => {
            Ok(file_content)
        },
        Err(error) => {
            let err = FileReadError {};
            Err(err)
        }
    }
}