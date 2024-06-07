// Write a Rust function that calculates the circumference of a circle using the constant PI and a given diameter.


use std::io;

const PI:f64 = 3.141592653589793;

fn main() {

    let mut r = String::new();
    io::stdin().read_line(&mut r).expect("Failed to read");
    let r:f64 = r.trim().parse().expect("Not a number");
    
    println!("The circle circumference is {}", calculate_circumference(r) );
}


fn calculate_circumference(r:f64) -> f64 {
    2.0*PI*r
}