use std::collections::HashMap;

fn main() {
    // let blue = String::from("Blue");
    // let yellow = String::from("Yellow");


    // let mut scores: HashMap<String,i32> = HashMap::new();

    // scores.insert(blue, 10);
    // scores.insert(yellow, 50);


    // let team_name = String::from("Blue");
    // let score: Option<&i32> = scores.get(&team_name);

    let text = "hello world wonderfull world";

    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}" , map)
}


