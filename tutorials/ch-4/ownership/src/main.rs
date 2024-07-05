/*

Owenership Rules

1) Each value in Rust has an owner
2) There can only be one owner at a time
3) When the owner goes out of scope, the value will be dropped


*/


fn main() {
    
    
    { // s, s2 is not valid
        let s: &str = "hello"; // s id valid (stack)
        let s2 = String::from("Hello"); // s2 valid (heap)
    }  // s , s2 is not valid after


    let x = 5;
    let y = x; // copy

    let s1 = String::from("Hello");
    let s2 = s1; // move (not shallow copy)
    // println!("{s1}, world!"); // panic


    let s3 = String::from("Hello");
    // takes_ownership(s3);
    // println!("{s3}, world!"); // panic
    borrowing(&s3);
    println!("{s3}, world!"); // not panic


    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;
    // cannot borrow `s` as mutable more than once at a time

    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // cannot borrow `s` as mutable because it is also borrowed as immutable

    /*
    
    Two or more pointers access the same data at the same time.

At least one of the pointers is being used to write to the data.

There’s no mechanism being used to synchronize access to the data.
     */

    /*
    
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

     */

    /*
    
    The Rules of References
Let’s recap what we’ve discussed about references:

At any given time, you can have either one mutable reference or any number of immutable references.
References must always be valid.

     */

    // Slice types
//     let mut s = String::from("hello world");

//     let word = first_word(&s);

//    // s.clear(); // error!

//     println!("the first word is: {word}");
let my_string = String::from("hello world");

// `first_word` works on slices of `String`s, whether partial or whole
let word = first_word(&my_string[0..6]);
let word = first_word(&my_string[..]);
// `first_word` also works on references to `String`s, which are equivalent
// to whole slices of `String`s
let word = first_word(&my_string);

let my_string_literal = "hello world";

// `first_word` works on slices of string literals, whether partial or whole
let word = first_word(&my_string_literal[0..6]);
let word = first_word(&my_string_literal[..]);

// Because string literals *are* string slices already,
// this works too, without the slice syntax!
let word = first_word(my_string_literal);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// fn first_word(s:&String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

fn takes_ownership(some_string:String) {
    println!("{some_string}")
}

// borrowing (not take ownership)
fn borrowing(some_str: &String) {
    println!("{some_str}")
}
