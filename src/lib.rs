mod pack;
mod enumerate;
mod implements;
// mod range_match;
use pack::gcd as gcd;

fn main() {
    gcd::test();
    // range_match::test();
    enumerate::test();
    implements::test();
}