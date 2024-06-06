// Write a Rust program that checks if a number is even or odd and prints the result.

use std::io;

fn main() {

   println!("Please enter a number");
   let mut num1 = String::new();

    io::stdin().read_line(&mut num1).expect("Failed to read number");
    
    let num1 :i32 = num1.trim().parse().expect("It is not a number");

    match num1 % 2 {
        0 => println!("{} is even number" , num1),
        1 => println!("{} is odd number" , num1),
        _ => (),
    }
   

}
