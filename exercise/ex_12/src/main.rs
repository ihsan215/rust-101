// Write a Rust function that takes a reference to a variable as a parameter and modifies its value.

fn main() {
    let mut str1 = String::from("Hello");
    take_ref_and_modify(&mut str1);
    println!("{}", str1);
    
}


fn take_ref_and_modify(str1:&mut String) {
    str1.push_str(", wolrd!");
}