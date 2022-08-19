use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let res = (2 * a - 1).max(2 * b - 1).max(a + b);
    println!("{}", res);
}
