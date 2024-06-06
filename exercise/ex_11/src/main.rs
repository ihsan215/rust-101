// Write a Rust program that creates two variables p and q, assigns a value to p, then assigns p to q and try to use p again.

fn main() {
    let mut p = 5;
    let mut q = 10;
    let mut temp = p;


    println!("p is {} , and q is {}", p,q);
    p = q;
    q= temp;
    println!("After change");
    println!("p is {} , and q is {}", p,q);
    

}
