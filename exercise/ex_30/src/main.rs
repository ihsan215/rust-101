// Write a Rust function to check if a number is prime.


use std::io;

fn main() {

    println!("Please enter a number : ");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read");
    let num:u32 = num.trim().parse().expect("Not a number");


    if check_is_prime(num) {
        println!("The number is prime");
    } else{
        println!("The number is not prime");
    }
}


fn check_is_prime(num:u32) -> bool {

    if num < 1 {
        return false;
    }

    for i in 2..num{
        if num % i == 0{
            return false;
        }
    }
    true
}