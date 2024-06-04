
#[derive(Debug)]
struct Reactangle {
    width:i32,
    height:i32,
}


fn main() {

    let width1 = 30;
    let height1 = 50;
    let rect1 = (50,30);

    let rect_st = Reactangle {
        width:50,
        height:30,
    };

    

    println!("The area of rectangle is {}" , area(width1, height1));
    println!("The area of rectangle is {}" , area_tuple(rect1));
    println!("The area of rectangle is {}" , area_st(&rect_st));
    println!("{:?}" , rect_st);
    dbg!(&rect_st);
}

fn area(width:usize,height:usize) -> usize {
    width*height
}

fn area_tuple(dimensions:(usize,usize)) -> usize {
    dimensions.0 * dimensions.1
}

fn area_st(rect:&Reactangle) -> i32 {
    rect.height*rect.width
}