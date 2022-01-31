#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}

fn main() {
    let ip_1 = IpAddr::V4(String::from("2.2.2.2"));
    let ip_2 = IpAddr::V6(String::from(":::1"));
    println!("ip1 {:?}\nip_2 {:?}", ip_1, ip_2);
}