pub fn test() {
    let o_1 = Option::Some(String::from("ddd"));
    // let o_2: Option<String> = None;
    let o_2 = Some(String::from("gghfgh"));
    println!("opt {:?}, {:?}", o_1, o_2);
    let res = match o_2.clone() {
        Some(s) => s,
        // None => String::from("no_ne"),
        _ =>  String::from("")
    };
    println!("o2 {}", res);

    match o_2.clone() {
        Some(s) => println!("o2 is string '{}'", s),
        None => println!("o2 is none")
    }

    let o_3 = Some(3);
    // let o_3: Option<u32> = None;

    // match o_3.clone() {
    //     Some(3) => println!("o3 is 3"),
    //     Some(k) => println!("o3 is '{}'", k),
    //     None => println!("o3 is none")
    // }

    match o_3.clone() {
        Some(3) => println!("o3 is 3"),
        _ => ()
    }

    if let Some(3) = o_3  { println!("o4 is 3") };
}