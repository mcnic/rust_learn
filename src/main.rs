#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let ip_1 = IpAddr {address: String::from("2.2.2.2"), kind: IpAddrKind::V4};
    let ip_2 = IpAddr {address: String::from(":::1"), kind: IpAddrKind::V6};
    println!("ip1 {:?}\nip_2 {:?}", ip_1, ip_2);
}