// Write a Rust program that declares a boolean variable is_sunny and sets it to true if it's sunny and false otherwise.

fn main() {
 
    let is_sunny: bool;


    let weather_condition = "sunny";

    
    if weather_condition == "sunny" {
        is_sunny = true;
    } else {
        is_sunny = false; 
    }

   
    if is_sunny {
        println!("It's sunny today!");
    } else {
        println!("It's not sunny today.");
    }
}
