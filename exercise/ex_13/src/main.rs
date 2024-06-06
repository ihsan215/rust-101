// Write a Rust function that takes two string references and returns the smallest one.

fn main() {
    let string1 = String::from("Helloooo");
    let string2 = String::from("World");
    
    let result = compare_strs(&string1, &string2);
    
    println!("The smallest string is: {}", result);
}


fn compare_strs(str1: &String, str2: &String) -> String {
    if str1.len() >= str2.len() {
        str2.clone()
    } else {
        str1.clone()
    }
}