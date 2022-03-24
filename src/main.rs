// trait MyTrait {
// }

fn largest<T>(list: &[T]) -> &T
    where T: PartialOrd {
    let mut larg = &list[0];
    for i in list {
        if i > &larg {
            larg = i;
        }
    }

    larg
}


fn main() {
    let list_1 = [5, 10, 2, 100, 1];
    let max_1 = largest(&list_1);
    println!("max1={:?}", max_1);

    let list_2 = ['a', 'b', 'c'];
    let max_2 = largest(&list_2);
    println!("max2={:?}", max_2);
}