mod pract_8_3_1;

use pract_8_3_1::get_data_of_list;

fn main() {
    let mut vec = vec![90, 101, 10, 10, 10, 80, 20, 20, 40, 20, 27, 10, 10];
    let res = get_data_of_list(&mut vec);
    println!("{:?}", res);
}