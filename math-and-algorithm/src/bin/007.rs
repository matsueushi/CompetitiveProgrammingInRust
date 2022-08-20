use num::Integer;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }
    let res = n / x + n / y - n / x.lcm(&y);
    println!("{}", res);
}
