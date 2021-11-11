enum IpAddrKind {
    V4,
    V6,
}

// more concise way
enum IpAddrStr {
    V4(String),
    V6(String),
}

// or multiple types
enum IpvAddrMult {
    V4(u8, u8, u8, u8),
}

// or with struct
struct Ipv4Addr;

enum IpAddrStruct {
    V4(IpAddrKind),
}

struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

// or with variety of types
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    let four = IpAddrKind::V4;
    route(four);
    // or
    let six = IpAddrKind::V6;
    route(six);

    let m = Message::Write(String::from("Hello"));
    m.call();

    let home = IpAddr {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStr::V6(String::from("::1"));

    let home = IpvAddrMult::V4(127, 0, 0, 0);
}

fn route(ip: IpAddrKind) {}
