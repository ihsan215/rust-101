
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String)
}

struct IpAddr {
    kind: IpAddrKind,
    address:String,
}


enum Message {
    Quit,
    Move {X:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}

impl  Message {
    fn some_function() {
        println!("Let's Get Rusty!");
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // let localhost = IpAddr {
    //     kind:IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    let localhost = IpAddrKind::V4(127,0,0,1);
}

fn route(ip_kind: IpAddrKind) {

}