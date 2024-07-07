use std::{collections::HashMap, default};



fn main() {
    // let mut scores: HashMap<String, i32> = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);
    // println!("{score:?}");
    // println!("{:?}" ,team_name);

    // for (key, value) in &scores{
    //     println!("The key is: {key}");
    //     println!("The value is : {value}");
    // }

    
    // scores.entry(String::from("Red")).or_insert(30);


    // for (key, value) in &scores{
    //     println!("The key is: {key}");
    //     println!("The value is : {value}");
    // }


    let text = "hello world wonderful world world";

    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}")
}
