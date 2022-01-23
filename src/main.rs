use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();
    println!("rand {}", y);
}