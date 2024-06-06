// Write a Rust program that prints all elements of an array using a for loop.

fn main() {
    let arr1:[i32;5] = [1,2,3,4,5];

    for item in arr1.iter() {
        println!("item is {}" , item);
    }
}
