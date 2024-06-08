// Write a Rust program that defines a struct Point with fields x and y. Now write a function to calculate the distance between two points.


use std::num;

#[derive(Debug)]
struct Point {
    x:isize,
    y:isize,
}

#[derive(Debug)]
struct Coordinates {
    point_1 : Point,
    point_2 : Point,

}

impl Coordinates {
    fn calculate_distance(&self) -> f64 {
        let distance_x:isize = self.point_1.x - self.point_2.x;
        let distance_y:isize = self.point_1.y - self.point_2.y;

        let distance_sq = distance_x*distance_x+distance_y*distance_y;
        let distance = (distance_sq as f64).sqrt();
        distance
    }
}

fn main() {
    let coords: Coordinates = Coordinates {
        point_1: Point {
            x:3,
            y:5
        },
        point_2 : Point {
            x:12,
            y:15,
        }
    };
    let distance:f64 = coords.calculate_distance();
    println!("Point 1 : {:?}, Point 2: {:?}, distance is {:.1}" , coords.point_1,coords.point_2,distance);
}
