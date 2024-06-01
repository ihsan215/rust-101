/*
    The Slice Type

    Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

*/


fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!


    // String slice
    let s2 = String::from("hello world");

    let hello = &s2[0..5];
    let world = &s2[6..11];

    println!("{}", hello);
    println!("{}", world);
}




fn first_word(s:&String) -> usize {
    // let bytes = s.as_bytes();

    // for (i, &item) in bytes.iter().enumerate(){
    //     if item == b' '{
    //         return  i;
    //     }
    // }

    
    // s.len()

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}