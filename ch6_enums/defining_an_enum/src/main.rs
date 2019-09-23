enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String)
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("Hello, world!");

    let some_number = Some(5);
    let some_string = Some("a string");

    //rust doesnt have null but it has the Option enum which has some and none on it
    let absent_number : Option<i32> = None;
}

fn route(ip_kind: IpAddrKind) { }