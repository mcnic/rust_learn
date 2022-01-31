#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, rec2: &Rectangle) -> bool {
        if self.width > rec2.width && self.height > rec2.height {
            return true;
        }
        if self.width > rec2.height && self.height > rec2.width {
            return true;
        }
        false
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let r1 = Rectangle {
        height:10,
        width: 20
    };
    let r2 = Rectangle {
        height:5,
        width: 10
    };
    let r3 = Rectangle {
        height: 15,
        width: 8
    };

    println!("area r1={}", r1.area());
    println!("r1 can hold r2={}", r1.can_hold(&r2));
    println!("r1 can hold r3={}", r1.can_hold(&r3));

    let r4 = Rectangle::square(10);

    println!("r4={:?}", &r4);
}