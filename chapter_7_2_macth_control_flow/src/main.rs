#[derive(Debug)] 
enum UsState {
    Alabama,
    Alaska,
 
}


enum Coin {
    Penny,
    Nichel,
    Dime,
    Quarter(UsState),
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nichel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// Matching with Option<T>

fn plus_one(x:Option<i32>) {
    match x {
        None => None,
        Some(i) => Some(i+1),
        
    };
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
fn main() {

    value_in_cents(Coin::Quarter(UsState::Alaska));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
    println!("Hello, world!");
}
