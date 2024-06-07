// Write a Rust function that accepts a mutable reference to a counter variable and increments it by a specified amount.



fn main() {
    let mut num:isize = 12;

    println!("The number is {}" , num);
    change_val(&mut num);
    println!("After incremention is {}" , num);

}


fn change_val(num:&mut isize) {
    *num += 1;
}