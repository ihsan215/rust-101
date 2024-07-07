// struct User {
//     username: String,
//     email:String,
//     active:bool,
//     sign_in_count:u64,
// }

// // Tupple Structs
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    fn area(&self) -> u32 {
        self.width * self.height
    }

   fn can_hold(&self, rect:&Rectangle) -> bool {
    self.width > rect.width && self.height > rect   .height
   } 
}


// Associated Functions
impl Rectangle {

    fn square(size:u32) -> Self {
        Self {
            width:size,
            height:size
        }
    }
    
}

fn main() {

    // let mut user1:User = User {
    //     username: String::from("Ali İhsan"),
    //     email: String::from("aliihsantas34@gmail.com"),
    //     active:true,
    //     sign_in_count:1
    // };

    // let name = user1.username;
    // user1.username = String::from("İhsan");
    // println!("Old name : {name} , New name : {:?}",user1.username);

    // let user2 = build_user(String::from("Ali"),String::from("deneme@deneme.com"));

    // let user3 = User {
    //     username: String::from("ali"),
    //     email: String::from("ali123@gmail.com"),
    //     ..user2
    // };

    // let black = Color(0, 0, 0);
    // let origin = Point(0, 0, 0);

    let rect = Rectangle {
        width: 30,
        height:50
    };

    let rect2 = Rectangle {
        width: 20,
        height:30
    };


    // let area = area(&rect);
    let area = rect.area();


    println!("The rectangle is {:?}", rect);
    println!("The area is {}" , area);

    println!("{:?} can hold {:?} : {}" ,rect,rect2,rect.can_hold(&rect2) );

    let sq = Rectangle::square(21);
    
    println!("The squarea is {:?}", sq);

}


// fn build_user(username:String,email:String) -> User {
//     User {
//         username: username,
//         email: email,
//         active:true,
//         sign_in_count:1
//     }

// }

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height

}