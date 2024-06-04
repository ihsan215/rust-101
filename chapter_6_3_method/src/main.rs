#[derive(Debug)]
struct Reactangle {
    width:u32,
    height:u32,
}

impl Reactangle {
    // The &self is actually short for self: &Self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Note that we can choose to give a method the same name as one of the struct’s fields. For example, we can define a method on Rectangle that is also named width
    fn width(&self) -> bool {
        self.width > 0
    }

 

    // Associated functions
    // Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct. 
    fn square(size:u32) -> Self {
        Self {
            width:size,
            height:size,
        }
    }

}

// Each struct is allowed to have multiple impl blocks. 
impl Reactangle {
    fn can_hold(&self, other:&Reactangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {

    let rect1 = Reactangle {
        width:30,
        height:50,
    };

    
    let rect2 = Reactangle {
        width:10,
        height:10,
    };

    
    let rect3 = Reactangle {
        width:30,
        height:80,
    };
    
    let sq = Reactangle::square(3);

    println!("The area of the reactangle is {}" , rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("Sq {:?}" , sq);
}
