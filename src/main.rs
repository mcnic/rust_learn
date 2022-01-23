use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("input number\n");
    let mut guess_num: i32;

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("error!");
        // println!("you tape {}", guess);
        guess_num = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess_num.cmp(&secret_number) {
            Ordering::Less => println!("too small ({})", secret_number),
            Ordering::Greater => println!("too great ({})", secret_number),
            Ordering::Equal => {
                println!("catch it!");
                return;
            }
        }
    }
}