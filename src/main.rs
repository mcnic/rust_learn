fn main() {
    let mut s1 = String::from("строка");
    s1.push('Я');
    let s2 = &s1[..];
    let s3 =  "еще строка".to_string() + &s1;
    let s4 = format!("{}+{}", "еще строка".to_string(), s1);
    println!("{}-{}-{}-{}", s1, s2, s3, s4);
}