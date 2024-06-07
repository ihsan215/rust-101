// Write a Rust program that implements two functions to find the maximum and minimum numbers in an array

fn main() {
    let numbers = [12,31,21,3,1,2,45,12,312,12,5];

    let max_num = find_max(&numbers);
    let min_num = find_min(&numbers);

    println!("The array is {:?}" , numbers);
    println!("The max of array is {}" , max_num);
    println!("The min of array is {}" , min_num);

}


fn find_max(numbers: &[i32]) -> i32 {

    let mut max_num = numbers[0];;
    for &num in numbers.iter(){
        if(max_num < num) {
            max_num = num; 
        }
    }

    max_num
}

fn find_min(numbers: &[i32]) -> i32 {
    let mut min_num = numbers[0];;
    for &num in numbers.iter(){
        if(min_num > num) {
            min_num = num; 
        }
    }

    min_num
}