//  Write a Rust program that accepts two numbers from the user, adds them together, and displays the result.

use std::io;

fn main() {

    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Please enter first number");

    io::stdin().read_line(&mut num1).expect("Failed to read first number");

    let num1:i32 = num1.trim().parse().expect("Input not an integer");

    println!("Please enter second number");

    io::stdin().read_line(&mut num2).expect("Failed to read first number");

    let num2:i32 = num2.parse().expect("Input not an integer");


    println!("The summation is {}" , num1+num2);
    
}
