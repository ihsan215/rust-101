// Rust groups errors into two major categories: recoverable and unrecoverable errors. 

/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}

*/

use std::fs::File;
use std::io::ErrorKind;

fn main() {



    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {e:?}"),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {other_error:?}");
    //         }
    //     },
    // };


    // let greeting_file = File::open("hello.txt").unwrap();

    // let greeting_file = File::open("hello.txt")
    // .expect("hello.txt should be included in this project");
}


