//  Write a Rust program that creates an array of numbers containing 5 integers. Iterate over the array and print each element.


fn main() {

    let arr: [i32;5] = [1,2,3,4,5];

    for item in arr.iter(){
        println!("{item}");
    }

  
}
