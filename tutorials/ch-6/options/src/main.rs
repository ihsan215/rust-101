// #[derive(Debug)] // so we can inspect the state in a minute
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }


fn main() {
    // let some_number = Some(5);
    // let some_char = Some('e');

    // let absent_number: Option<i32> = None;

    // let x : i8 = 5;
    // let y: Option<i8> = Some(5);
    // let y: Option<i8> = None;


    // let sum:i8 = x + y.unwrap_or(0);

    // println!("The sum is {sum}");
    // value_in_cents(Coin::Quarter(UsState::Alaska));
    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    let some_value:Option<i32> = Some(3);
    if let Some(3) = some_value {
        println!("three");
    } 
}


// fn value_in_cents(coin:Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucy penny!");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {state:?}");
//             25
//         }
//     }
// }

// fn plus_one(some: Option<i32>) -> Option<i32> {
//     match some {
//         Some(i) => Some(i+1),
//         _ => None
        
//     }
// }