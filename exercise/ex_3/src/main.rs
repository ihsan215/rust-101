// Write a Rust program that declares a mutable variable counter and initializes it with 0. Then increase it by 1 and decrease it by 1. At the end, print the variable value for each stage.

fn main() {
   
   println!("Start Loop");
   let mut counter = 0;

   while  counter < 10 {
       println!("Counter is {}" , counter);
       counter += 1;
   }

   loop {
 
    println!("Counter is {}" , counter);
    if counter <= 0 {
        break;
    }
    counter -= 1;

  
   }
     
   println!("Start end");
}
