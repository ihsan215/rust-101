// Write a Rust function that takes ownership of a string and prints it.

fn main() {

    let s = String::from("hi");
    take_ownership(s);
    // println!("{s}");
   
}


fn take_ownership(str1:String) {
    println!("{}",str1);
}