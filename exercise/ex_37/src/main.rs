// Write a Rust function that takes ownership of a vector and returns its length.


fn main() {
    let my_vector = vec![1, 2, 3, 4, 5]; 
    let vector_length = get_vector_length(my_vector);
    println!("Length of the vector: {}", vector_length);

}


fn get_vector_length(v:Vec<u32>) -> usize {
    v.len()
}