#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn route(ip: &IpAddr) -> String {
    // &ip.address//.push_str(ip.address)
    format!("'{:#?}' {}", &ip.kind, &ip.address)
}

fn main() {
    let ip_v4 = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    let ip_v6 = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
    println!("{:#?}", ip_v4);
    println!("{:#?}", route(&ip_v6));
}

