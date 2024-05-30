fn main() {
    println!("Start main");
    another_function(5);
    print_labeled_measurement(5, 'h');


    let y = {
        let x = 3;
        x+1
        //  Note that the x + 1 line doesn’t have a semicolon at the end, which is unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons.
    };

    println!("The value of y is {y}");

    let z = five();
    let v = plus_one(z);
    println!("The value of z is {z}");
    println!("The value of v is {v}");

}

fn another_function(x:i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value:i32,unit_label:char){
    println!("The measurement is: {value}{unit_label}");

}

fn five() -> i32 {
    5
}

fn plus_one(x:i32) -> i32 {
    x+1
}
// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value. Let’s look at some examples.

// The let y = 6 statement does not return a value, so there isn’t anything for x to bind to