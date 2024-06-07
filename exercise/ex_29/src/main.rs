// Write a Rust function that calculates the area of a rectangle given its width and height.

fn main() {

    let width = 12.3;
    let height = 23.2;
    let area = calculate_area(width,height);
    println!("Width is {} , height is {} and area is {}" , width,height,area);
}


fn calculate_area(width:f64, height:f64) -> f64 {
    width * height
}