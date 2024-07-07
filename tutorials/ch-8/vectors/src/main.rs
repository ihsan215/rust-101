
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {

    // let mut v: Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);

    // let mut v2: Vec<i32> = vec![1,2,3,4,5];
    
    // let third = &v2[2];
    // // v2.push(6); // error
    // println!("The third element is {}" , third);
    // v2.push(6); // not error

    // match v2.get(2) {
    //     Some(third) =>    println!("The third element is {}" , third),
    //     None =>  println!("There is no third element"),
    // }


    let mut v = vec![1,2,3,4,5];

    for i in &v{
        println!("{}" , i);
    }

    for i in &mut v{
        *i += 50;
    }

    for i in &v{
        println!("{}" , i);
    }

    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
