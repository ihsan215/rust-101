fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // sum takes ownership of the iterator we call it on.
    // let total: i32 = v1_iter.sum();
    
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    
    println!("v2: {:?}", v2);



    for val in v1_iter {
        println!("Got: {val}");
    }
}
