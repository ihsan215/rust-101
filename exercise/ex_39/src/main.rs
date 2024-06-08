// Write a Rust program that defines a struct Rectangle with fields width and height. Write a function to calculate the area of a rectangle instance.


struct Reactangle {
    width: usize,
    height:usize,
}


impl Reactangle {
    fn calculate_area(&self) -> usize {
        self.width * self.height
    }
}

fn main() {
    let rect1:Reactangle = Reactangle {
        width: 23,
        height: 10,
    };

    println!("The area of rectangle is {}" , rect1.calculate_area() )
}
