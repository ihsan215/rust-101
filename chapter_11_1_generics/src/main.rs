// Generics allow us to replace specific types with a placeholder that represents multiple types to remove code duplication. 

struct Point<T> {
    x:T,
    y:T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point2<T,U> {
    x:T,
    y:U
}

fn main() {
    
    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {result}");

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largest char is {result}");

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

}


// not work 
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }