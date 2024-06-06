 // Write a Rust function that takes an optional integer and prints its value if it exists.

fn main() {

    check_NaN(Some(12u32));
    check_NaN(None);
}


fn check_NaN(val:Option<u32>)  {

    match val {
        Some(val) => println!("The value is {:?}" , val),
        None => println!("The value is absent"),
    }
}