// enum IpAddrKind {
//     V4,
//     V6,
// }

// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);
// }

// fn route(ip_kind: IpAddrKind) {}


fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}