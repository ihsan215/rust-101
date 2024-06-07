// Write a Rust function that takes a string and returns the reverse of the string.


fn main() {
    let s = "ali ihsan tas";
    let reversed = reverse(s);
    println!("{} reversed is {}" , s ,reversed);
}


fn reverse(s:&str) -> String {
    s.chars().rev().collect()
}