mod pack;
mod enumerate;
mod implements;
// mod range_match;
use pack::gcd as gcd;

#[test]
fn test_all() {
    gcd::test();
    // range_match::test();
    enumerate::test();
    implements::test();
    assert_eq!(1, 1);
}