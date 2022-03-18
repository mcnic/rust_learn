fn largest(list: &[i32]) -> i32 {
    if list.len() == 0 {
        return 0;
    }

    let mut larg = list[0];
    for i in list {
        if i > &larg {
            larg = *i;
        }
    }

    larg
}


fn main() {
    let list_1 = [5, 10, 2, 100, 1];
    let max_1 = largest(&list_1);
    println!("max1={:?}", max_1);
}