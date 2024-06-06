// Write a Rust program that handles the result of a division operation, using pattern matching to distinguish between success and failure.

fn main() {
    match division(5,2) {
        Ok(val) => println!("{} / {} = {}" ,5,2,val),
        Err(err) => println!("Err is {}" , err),
    }

    match division(5,0) {
        Ok(val) => println!("{} / {} = {}" ,5,2,val),
        Err(err) => println!("Err is {}" , err),
    }
}


fn division(num1:i32, num2:i32) -> Result<i32, &'static str> {
   if num2 == 0 {
    Err("Dibision by zero")
   }
   else{
    Ok(num1/num2)
   }
}