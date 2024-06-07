// Write a Rust program that implements a function to calculate the nth Fibonacci number.


use std::io;

fn main() {

    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read");
    let num = num.trim().parse().expect("Not a number");

    println!("The {} nth fibo is {}" , num,calcualte_fibonacci(num));
}

fn calcualte_fibonacci(num:i32) -> i32{

    if num < 0 {
        return 0;
    } else if num == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    let mut fibo = 0;

    for i in 2..=num {
        fibo = a + b;
        a = b;
        b = fibo;
    }

    fibo
}   
