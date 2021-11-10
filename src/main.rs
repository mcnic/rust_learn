// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
//
// fn change_str(s: &mut String) -> usize {
//     s.push_str(", 123");
//     s.len()
// }
//
// fn clone_str(s: String) -> usize {
//     s.len()
// }

// #[derive(Debug)]
// struct Color (i32, i32, i32);
//
// #[derive(Debug)]
// struct User {
//     name: String,
//     surname: String,
//     age: Color
// }
//
// struct Null ();
//
// fn get_age(user: &User) -> &String {
//     &user.surname
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        // if self.width * self.height > rect.width * rect.height { true } else { false }
        self.width > rect.width  && self.height > rect.height
    }

    fn square(size: u32) -> Rectangle {Rectangle{
        height: size,
        width: size
    }}
}

fn main() {
    // let mut s1 = String::from("hello");
    // let s3 = takes_and_gives_back(s2);
    // let len = calculate_length(&s1);
    // let len = change_str(&mut s1);
    // let len = clone_str(s1.clone());
    // let ll = String::from("weff r23fds dfsdfs");
    // for (i, &el) in ll.as_bytes().iter().enumerate() {
    //     println!("{} {}", i, el);
    // }

    // let _col = Color(4, 10,43);
    //
    // let us = User {
    //     name: String::from("123"),
    //     surname: String::from("mlkdf dfkjdkfh sdfhjsdfhsj"),
    //     age: _col
    // };
    //
    // println!("{} {} {:#?}", get_age(&us), us.age.0, us);
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("{:#?}", Rectangle::square(10));
}

