// Write a Rust program that defines a string variable name and assigns it your name. Print a greeting message using the name variable.

fn main() {
    let mut name = String::from("Ali İhsan Tas");
    greeting(&mut name);
    println!("{}" , name);
}


fn greeting(greets:&mut String) {
    greets.push_str(", welcome to the Rust programming world!");

}