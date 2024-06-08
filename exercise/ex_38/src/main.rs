// Write a Rust function that borrows a vector and returns the first element.


fn main() {
    let my_vector = vec![12, 2, 3, 4, 5];

    println!("{}", get_first_element(&my_vector));
    println!("Used 'my_vector' after borrowing: {:?}", my_vector);

}

fn get_first_element(v : &Vec<usize>) -> usize {
    v[0]
}