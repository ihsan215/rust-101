
#![allow(unused)]
use std::{error, fs::File};
use std::io::ErrorKind;


use std::io::{self, Read};

fn main() {
    
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(err) => match err.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem with creating the file : {:?}" ,e),
                
    //         },
    //         other_error => panic!("Problem with creating the file : {:?}" ,other_error),
    //     }
    // };
   
}


// fn a(){
//     b();
// }

// fn b() {
//     c(22);
// }

// fn c(num:i32){
//     if num == 22 {
//         panic!("Don't pass in 22");
//     }
// }


fn read_username_from_file() -> Result<String, io::Error> {
    // let username_file_result = File::open("hello.txt");

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut username = String::new();

    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
    
}