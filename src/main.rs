fn main() {
    let mut v1 = vec![1, 2, 4];
    // let link_v1 = &v1[0];
    // v1.push(2);
    println!("{:?}", v1);
    v1[0] = 10;
    for i in &mut v1 {
        *i += 50;
    }
    println!("vvv2 {:?}", v1);
}