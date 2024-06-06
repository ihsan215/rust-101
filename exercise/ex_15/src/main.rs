//  Write a Rust program that defines an enum Color with variants representing different colors.


#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let red = Color::Red;
    let green = Color::Green;
    let blue = Color::Blue;

    println!("{:?}" , red);
    println!("{:?}" , green);
    println!("{:?}" , blue);
    
}
