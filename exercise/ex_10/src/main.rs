// Write a Rust program that defines a function that calculates the factorial of a given number and returns the result.


use std::io;

fn main() {


    println!("Please enter a number far calculate factorial");
    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("Failed to read");

    let num:u32= num.trim().parse().expect("Not a number");

    let fac_num = fac(num);
    println!("{}! =  {}" ,num,fac_num);
}



fn fac(num:u32) -> u32 {
    if num <= 1{
        1
    }
    else {
        num * fac(num-1) 
    }
}