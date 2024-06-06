// Write a Rust function that returns a success value for positive numbers and an error value for negative numbers.
fn main() {

    match check_positive(5) {
        Ok(val) => println!("Result is {}" , val),
        Err(err) => println!("Err is {}" ,err),
    }

    match check_positive(-5) {
        Ok(val) => println!("Result is {}" , val),
        Err(err) => println!("Err is {}" ,err),
    }
}


fn check_positive(num:i32) -> Result<i32, &'static str> {

    if num >= 0{
        Ok(num)
    }
    else{
        Err("This is a negative number")
    }

}
