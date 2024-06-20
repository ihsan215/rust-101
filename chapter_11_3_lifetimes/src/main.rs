/*

Basic Lifespan Rules

1) Every reference has a lifetime: In Rust, every reference has a lifetime, which defines the length of time the reference can be used.

2) A reference must be valid throughout its lifetime: A reference must point to a data structure that is valid throughout its lifetime.
*/


fn longest<'a>(x:&'a str, y:&'a str) -> &'a str{
    if x.len() > y.len(){
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);

    println!("The longest string is {result}");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let my_string = String::from("hello world");
    let word = first_word(&my_string);
    println!("The first word is: {}", word);
}
