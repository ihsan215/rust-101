
/*

-) Ownership is a set of rules that govern how a Rust program manages memory. 


HEAP vs STACK

Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways. 

STACK : 

1) The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out. 

2) All data stored on the stack must have a known, fixed size. 

HEAP :

The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating).

Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.

STACK VS HEAP 

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there.


When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.



Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses. Once you understand ownership, you won’t need to think about the stack and the heap very often, but knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does.

*/

/*

    Ownership Rules

    First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

    1) Each value in Rust has an owner.
    2) There can only be one owner at a time.
    3) When the owner goes out of scope, the value will be dropped.

*/

// String type stored on heap

fn main() {


   
    // This kind of string can bu mutated:
    let mut s:String = String::from("Hello");
    s.push_str(", world!");

    println!("{}",s);

 

    // s1 was moved into s2
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}",s1);
    // println!("{}",s2);

    // clone data on the heap
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}",s1);
    println!("{}",s2);

    // Stack-Only Data

    /*

    The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.
     */
    
    // copy
    let y = 5;
    let x = y;

    println!("{}",x);
    println!("{}",y);


    // Ownership and functions

    /*
    The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.
     */
    let s3 = String::from("hello");  // s comes into scope

    takes_ownership(s3);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x); 


    let s4 = gives_ownership();         // gives_ownership moves its return
    // value into s1

    let s5 = String::from("hello");     // s5 comes into scope

    let s6 = takes_and_gives_back(s5);  // s5 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s6

    let s7 = String::from("experiment");
    let (s8,len) = calculate_length(s7);
    println!("The lenght of {} is {}.",s8,len);
    /*
    While this works, taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.

    But this is too much ceremony and a lot of work for a concept that should be common. Luckily for us, Rust has a feature for using a value without transferring ownership, called references.
     */

}


fn takes_ownership(some_tring:String) {
    println!("{}", some_tring);
}

fn makes_copy(some_integer:i32){
    println!("{}", some_integer);
}


fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s:String) -> (String,usize) {
    let length = s.len();
    (s,length)
}