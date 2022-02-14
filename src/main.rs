use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(100);
    println!("vec1 {:?}", v1);

    let mut v2 = vec![1, 2, 4];
    v2.push(5);
    println!("vec2 {:?}", v2);

    // let vvv2: &i32 = &v2[20];
    let vvv2 = &v2.get(2);
    let type_vvv2 = match(vvv2)  {
        Some(i) => i.to_string(),
        None => String::from("None")
    };
    println!("vvv2 {:?} {:?} {:?}", vvv2, type_of(vvv2), type_vvv2);
}