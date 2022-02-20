fn main() {
    for c in "строка".chars() {
        println!("{}", c);
    }
    for b in "строка".bytes() {
        println!("{}", b);
    }
}