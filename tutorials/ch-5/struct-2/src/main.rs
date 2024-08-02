// struct User {
//     username: String,
//     email:String,
//     sign_in_count:u64,
//     active:bool,
// }

#[derive(Debug)]
struct Reactangle {
    width: u32,
    height: u32
}

impl Reactangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other:&Reactangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Reactangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let mut user1 = User {
    //     email:String::from("deneme@gmail.com"),
    //     username: String::from("ihsan215"),
    //     active:true,
    //     sign_in_count:1
    // };

    // let  name = user1.username;
    // user1.username = String::from("ihsan123");

    // let user2 = build_user(String::from("deneö@.com"), String::from("deneme2"));

    // let user3 = User {
    //     email:String::from("deneme@gmail.com"),
    //     username: String::from("ihsan215"),
    //     ..user2
    // };


    // // Tuple struct
    // struct Color(i32,i32,i32);
    // struct Point(i32,i32,i32);


    let rect = Reactangle {
        width:30,
        height: 50
    };

    let rect2 = Reactangle {
        width:10,
        height: 20
    };

    let rect3 = Reactangle::square(12);
    println!("Rect is {:?}" , rect3);
    println!("Rect is {:?}" , rect);
    println!("The rect area is {}",rect.area());
    println!("The rect can hold {}",rect.can_hold(&rect2));

}

// fn build_user(email:String,username:String) -> User {
//     User {
//         email,
//         username,
//         active:true,
//         sign_in_count:1
//     }
// }