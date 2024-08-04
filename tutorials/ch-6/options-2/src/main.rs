enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}


fn main() {
   let some_number = Some(5);
   let some_string = Some("a string");

   let x: i8 = 5;
   let y: Option<i8> = Some(5);

   let sum = x + y.unwrap_or(0);

   let five = Some(5);
   let six = plus_one(five);
   let none = plus_one(None);


}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}",state);
            25
        },
    }
}

fn plus_one(x:Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
        _ => None
    }
}