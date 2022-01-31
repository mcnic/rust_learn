#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn main() {
    let ip_1 = IpAddr::V4(2 ,2, 2, 2);
    let ip_2 = IpAddr::V6(String::from(":::1"));
    println!("ip1 {:?}\nip_2 {:?}", ip_1, ip_2);

    let o_1 = Option::Some(String::from("ddd"));
    // let o_2: Option<String> = None;
    let o_2 = Some(String::from("gghfgh"));
    println!("opt {:?}, {:?}", o_1, o_2);
    let res = match o_2 {
        Some(s) => s,
        None => String::from("no_ne")
    };
    println!("o2 {}", res);
}