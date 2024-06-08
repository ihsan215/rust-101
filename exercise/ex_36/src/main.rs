// Write a Rust function that borrows a string and prints it.



fn main() {
    let s = String::from("hello");
    borrow_and_print(&s);
    println!("{s}");
   
}


fn borrow_and_print(str1: &String) {
    println!("{str1}");
}