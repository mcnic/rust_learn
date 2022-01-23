use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("secret is {}\n", secret_number);

    loop {
        println!("input number");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("error!");
        let guess_num: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
        println!("you tape {}", guess_num);

        match guess_num.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too great"),
            Ordering::Equal => {
                println!("catch it!");
                return;
            },
        }
    }
}