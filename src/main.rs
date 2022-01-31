#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddr {
    addr: String,
    ver: IpAddrKind
}

fn main() {
    let ip_1 = IpAddr {addr: String::from("2.2.2.2"), ver: IpAddrKind::V4};
    println!("ip1 {:?}", ip_1);
}