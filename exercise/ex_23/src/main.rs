// Write a Rust program that creates an integer variable temp and assign it a temperature value in Celsius. Convert it to Fahrenheit and print the result.

fn main() {
    
    let mut degree_in_c:i32 = 12;
    println!("Degree in celsius is {:.2} ",degree_in_c);
    println!("Degree in fahrenheit is {:.2} ",cel_to_fah(degree_in_c));

}

fn cel_to_fah(degree_in_c:i32) -> f64 {
    (degree_in_c as f64 * 9.0 / 5.0) + 32.0 
}