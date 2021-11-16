use std::fmt;

enum Message {
    Quit,
    Input (Option<i32>),
    // Move {x: i32, y: i32},
    Write (String),
    ChangeColor (i32, i32, i32)
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Message::Quit => write!(f, "_"),
            Message::Write(string) => write!(f, "'{}'", string),
            Message::ChangeColor(i0, i1, i2) => write!(f, "{}.{}.{}", i0, i1, i2),
            Message::Input(inp) => match inp {
                Some(inp_i32) => write!(f, "i32:{}", inp_i32),
                None => write!(f, "is None")
            }
        }
    }
}

fn is_important(mes: &Message) -> String {
    if let Message::Quit = mes {
        return String::from("yes")
    }

    String::from("no")
}

fn main() {
    println!("mes0 {}", Message::Quit);
    println!("mes1 {}", Message::Write(String::from("555")));
    println!("mes2 {}", Message::ChangeColor(1, 2, 3));
    println!("mes3 {}", Message::Input(Some("5".parse::<i32>().unwrap())));

    println!("imp {}", is_important(&Message::Quit));
}

