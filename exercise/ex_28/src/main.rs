// Write a Rust program that creates a vector of items containing different types of items. Print each item in the vector.

fn main() {

    let items: Vec<(&str,i32,char,bool)> = vec![("ali",15,'A',true),("ihsan",15,'b',false)];


    for (a,b,c,d) in &items {
        println!("{}, {}, {}, {}",a,b,c,d);
    }
}
