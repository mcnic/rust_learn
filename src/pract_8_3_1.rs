// Есть список целых чисел. Создайте функцию, используйте вектор и верните из списка:
// среднее значение;
// медиану (значение элемента из середины списка после его сортировки);
// моду списка (mode of list, то значение которое встречается в списке наибольшее количество раз;

use std::collections::HashMap;

pub fn get_data_of_list(list: &mut Vec<i32>) -> (f64, i32, i32) {
    list.sort();
    // println!("sort list {:?}", list);

    let mut sum: usize = 0;

    let mut moda_hash_map = HashMap::new();
    let len_list = &list.len();
    for i in list.into_iter() {
        sum += *i as usize;
        let count = moda_hash_map.entry(*i).or_insert(0);
        *count += 1;
    };
    let med = (sum as f64) / (*len_list as f64);
    let median = match list.get((*len_list as usize) / (2)) {
        Some(res) => *res,
        None => 0
    };

    let mut hash_vec: Vec<(&i32, &i32)> = moda_hash_map.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));
    let moda = match hash_vec.get(0) {
        Some(res) => *res.1,
        None => 0
    };
    // println!("hash_vec {:?} {:?}", hash_vec, moda);

    (med, median, moda)
}

#[test]
fn test_get_data_of_list() {
    let mut vec = vec![90, 101, 10, 60, 10, 80, 20, 20, 40, 20, 27];
    let res = get_data_of_list(&mut vec);
    // println!("{:?}", &res);
    assert_eq!(res, (43.45454545454545, 27, 3));
}
