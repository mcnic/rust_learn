use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("sectet is {}", secret_number);

    println!("input number\n");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("error!");
    let mut guess: u32 =  guess
        .trim()
        .parse()
        .expect("not a  number");
    println!("you tape {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too great"),
        Ordering::Equal => println!("catch it!"),
    }
}