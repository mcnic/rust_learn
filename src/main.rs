struct Rectangle {
    height: u32,
    width: u32
}

fn sq(r: &Rectangle) -> u32 {
    r.height * r.width
}

fn main() {
    let r = Rectangle{
        height:10,
        width: 20
    };

    println!("square={}", sq(&r));
}