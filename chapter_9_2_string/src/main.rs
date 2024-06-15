fn main() {

    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();

    let s = "initial contents".to_string();


    let s2 = String::from("initial content");


    let mut s3 = String::from("hello");
    s3.push_str("world");
    println!("s3 is {s3}");


    let mut s4 = String::from("Hello , ");
    let s5 = &s4;

    println!("s4 is {s4}");
    println!("s5 is {s5}");


    let hello = String::from("hello");
    let h =&hello[0..1];
    println!(" h is {h}");

    for c in hello.chars(){
        println!("{c}");
    }

    for c in hello.bytes(){
        println!("{c}");
    }

    
    
}
