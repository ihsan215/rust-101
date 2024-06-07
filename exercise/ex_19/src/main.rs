// Write a Rust program that declares a variable birth_year and calculates the age based on the current year and the user's birth year.

use std::io;
use chrono::prelude::*;

fn main() {

    println!("Enter your birth year !");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read");
    let num:i32 = num.trim().parse().expect("Not a number");

    let current_date = Utc::now();
    let current_year = current_date.year();

    println!("Your age is  : {} - {} = {}" , current_year,num,current_year-num);
  
}
