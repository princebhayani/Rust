use std::fs;

fn main() {
    let greeting_file_result = fs::read_to_string("hello.txt");

    match greeting_file_result {
        Ok(file_content) => {
            println!("File read successfully: {:?}", file_content);
        },
        Err(error) => {
            println!("Failed to read file: {:?}", error);
        }
    }
}