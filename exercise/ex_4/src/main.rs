// Write a Rust program that converts an integer to a string and vice versa and prints the result.

fn main() {
    
    let number1: i32 = 124;
    let string1 = "124";


    // convert int to string
    let number1 = number1.to_string();
    let string1:i32 = string1.parse().expect("failed to convert");;

    println!("Converted number is {}" , number1);
    println!("Converted string is {}" , string1);



}
