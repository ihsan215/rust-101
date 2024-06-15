// hash map

use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("score is {:?}",score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // overwrite
    scores.insert(String::from("Blue"), 25);
    println!("score is {:?}",scores);

    // Entry that represents a value that might or might not exist. Let’s say we want to check whether the key for the Yellow team has a value associated with it. If it doesn’t, we want to insert the value
    scores.entry(String::from("Green")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("score is {:?}",scores);

    let text = "hello world wonderful world";

    
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

}
