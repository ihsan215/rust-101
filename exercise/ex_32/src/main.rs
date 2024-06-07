// Write a Rust function to calculate the factorial of a given number.


use std::io;

fn main() {

    println!("Please enter a number to calculate factoriel");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read");
    let num:i32 = num.trim().parse().expect("Not a number");
    println!("The factorial is {}" , cal_fac(num));
   
}


fn cal_fac(num:i32) -> i32 {
    if num < 2 {
        return 1;
    }

    let mut fac = 1;

    for i in 2..=num {
        fac *= i;
    }

    fac
}