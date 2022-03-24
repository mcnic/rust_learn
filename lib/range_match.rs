use rand::Rng;
use std::io;
use std::cmp::Ordering;

pub fn test() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("input number\n");
    let mut guess_num: i32;

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("error!");
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