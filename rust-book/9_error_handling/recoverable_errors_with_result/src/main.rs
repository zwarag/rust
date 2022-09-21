use std::fs::File;
use std::io::{self, Read, ErrorKind};

static FILE_NAME: &str = "hello.txt";

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open(&FILE_NAME);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

fn main() {
    // let greeting_file_result = File::open(&FILE_NAME);

    // let greeting_file_result = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create(&FILE_NAME) {
    //             Ok(fc) => {
    //                 println!("File {} not found. Creating the file... done!", &FILE_NAME);
    //                 fc
    //             }
    //             Err(e) => panic!("Problem opening the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    let greeting_file = File::open(&FILE_NAME).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(&FILE_NAME).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
