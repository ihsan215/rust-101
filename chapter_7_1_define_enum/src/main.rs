
enum IpAddrKind {
    V4,V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr_en {
    V4(u8, u8, u8, u8),
        V6(String),
}


/*

This enum has four variants with different types:

Quit has no data associated with it at all.
Move has named fields, like a struct does.
Write includes a single String.
ChangeColor includes three i32 values.
*/
enum Message {
    Quit,
    Move {x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

// Option

/*
Option<T>, and it is defined by the standard library as follows:

enum Option<T> {
    None,
    Some(T),
}

*/

fn main() {

    let four:IpAddrKind = IpAddrKind::V4;
    let six:IpAddrKind = IpAddrKind::V6;


    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home2 = IpAddr_en::V4(127, 0, 0, 1);
    let loopback2 = IpAddr_en::V6(String::from("::1"));

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}
