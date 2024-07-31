// Ownership is a set of rules that govern how a Rust program manages memory.


/*

Ownweship Rules

1) Each value in Rust has an owner
2) There can only be one owner at a time
3) When the owner goes out the scope, the value will be dropped

*/


/*

The Rules of References

1) At any given time, you can have either one mutable reference or any number of immutable references
2) References must always be valid


*/
fn main() {
    // println!("START PROGRAM!");

    // string (on heap)
    // let mut s = String::from("Hello");
    // s.push_str(", world!");
    // println!("{s}");



    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("{s1}, world!");

    // let s = String::from("hello"); 
    // takes_ownership(s); 

    // let x = 5;
    // make_copy(x);    

    // let s1 = gives_ownership();

    // let s2 = String::from("hello");     

    // let s3 = takes_and_gives_back(s2);



    // println!("END PROGRAM!");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {s1} is {len}");

    let mut s = String::from("hello");

    change(&mut s);

    // Mutable reference rules
    // 1) Two or more pointers access the same data at the same time.
    // 2) At least one of the pointers is being used to write to the data.
    // 3) There's no mechanism being used to synchronize access to the data

    //  We also cannot have a mutable reference while we have an immutable one to the same value.
    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");


    // Dangling References

    // a pointer that references a location in memory that may have been given to someone else
    // let reference_to_nothing = dangle();

}



// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn calculate_length(s:&String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn takes_ownership(some_string:String) {
//     println!("{some_string}");
// }

// fn make_copy(some_int:i32) {
//     println!("{some_int}");
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("yours");
//     some_string
// }

// fn takes_and_gives_back(a_string:String) -> String {
//     a_string
// }