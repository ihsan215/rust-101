//  Write a Rust program that prints all even numbers from 1 to 20 using a while loop.

fn main() {
    
    let mut counter = 0;

    while counter <= 20 {

       
        if counter % 2 == 0 {
            println!("The counter is {}" , counter);
        }       
        counter += 1;

    }
}
