struct User {
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}

// Tupple Structs
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

// Unit - Like Structs
struct AlwaysEqual;


fn main() {

    let subkect = AlwaysEqual;

    let mut user1 = User {
        active:true,
        username: String::from("someuser123"),
        email:String::from("someex@ex.com.tr"),
        sign_in_count:1,
    };

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email:String::from("anaothermail@com.tr"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);


   


    println!("Hello, world!");
}


fn build_user(email:String,username:String) -> User {
    User {
        active: true,
        username:username,
        email:email,
        sign_in_count:1,
    }
}