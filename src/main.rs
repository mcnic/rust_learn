#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

impl IpAddr {
    fn Debug1(&self) -> String {
        // &format!("{}.{}.{}.{}", self.0, self.1, self.2, self.3)
        format!("{}",String::from("123"))
    }
}
fn route(ip: &IpAddr) -> String {
    // &ip.address//.push_str(ip.address)
    format!("'{:#?}'", &ip)
}

fn main() {
    let ip_v4 = IpAddr::V4(127, 0, 0, 1);
    let ip_v6 = IpAddr::V6(String::from("::1"));

    // println!("{}", ip_v6);
    println!("{}", route(&ip_v4));
}

