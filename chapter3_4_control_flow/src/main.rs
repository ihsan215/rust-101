fn main() {
    // if expression

    let number = 4;

    // the condition is must be bool
    if number < 5{
        println!("condition was true");
    } else{
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }   
    
    // using ig in a let statement
    let condition:bool = true;
    let number = if condition {5} else {6};
    println!("The value of number is {number}");

    /*
        Error
        
    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");

     */


    // Repetition with loops
    // loop, whille, and for.

    let mut counter = 0;

    // Unlike the other kinds of loops in Rust (while, while let, and for), loops can be used as expressions that return values via break.
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    // loop label
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaning = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;

    }

    println!("End count = {count}");

    // Conditional loop with while 

    let mut number = 4;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");


    let a  = [10,20,30,40,50];
    let mut index = 0;

    while  index < 5 {
        println!("the value is {}",a[index]);
        index += 1;
    }   

    for element in a {
        println!("the value is {element}");
    }

    for number in (1..=4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    
}

