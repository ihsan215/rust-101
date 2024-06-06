// Write a Rust program that defines a struct Person with fields like name and age.

#[derive(Debug)]
struct Person {
    name:String,
    age:u32,
}

fn main() {
    let person1 =  Person {
        name:String::from("ali ihsan"),
        age:25
    };


    println!("Person1 is {:?}" , person1);
}
