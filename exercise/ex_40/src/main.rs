// Write a Rust program that creates an enum Direction with variants representing different directions (e.g., North, South, East, West).

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {

    let north = Direction::North;
    let south = Direction::South;
    println!(" {:?} , {:?} ", north,south);
}
