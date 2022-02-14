#[derive(Debug)]
enum MyType {
    Str(String),
    Int(i32)
}

fn main() {
   let mut v1: Vec<MyType> = vec![
       MyType::Int(1),
       MyType::Int(2),
       MyType::Int(4),
       MyType::Str(String::from("str"))
   ];
   v1.push(MyType::Int(2));
   println!("{:#?}", v1);
}