
// Collection type : Vector , Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.
fn main() {

    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1,2,3];
    v.push(1);
    v.push(2);
    v.push(3);

    let item:&i32 = &v[2];
    
    println!("The third element is {item}");


    let item: Option<&i32> = v.get(2);
    match item{
        Some(item) => println!("The third element is {item}"),
        None => println!("There is no third element"),
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }


    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}
