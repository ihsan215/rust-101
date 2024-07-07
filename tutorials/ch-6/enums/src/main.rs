enum IpAddrKind{
    V4(u8,u8,u8,u8),
    V6(String),
}

struct  IpAddr {
    kind:IpAddrKind,
    address: String,
}

impl IpAddrKind {
    fn some_func() {
        println!("Let's Get Rusty!");
    }
}


fn main() {
    
    // let four:IpAddrKind = IpAddrKind::V4;
    // let six:IpAddrKind = IpAddrKind::V6;

    // let localhost = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };

    let localhost = IpAddrKind::V4(127, 0, 0, 1);

    IpAddrKind::some_func();
}


