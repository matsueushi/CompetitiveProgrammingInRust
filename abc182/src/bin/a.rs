use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let res = 2 * a + 100 - b;
    println!("{}", res);
}
