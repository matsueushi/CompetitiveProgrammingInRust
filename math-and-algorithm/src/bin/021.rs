use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        r: usize,
    }
    let mut res = 1;
    for i in r + 1..=n {
        res *= i;
    }
    for j in 1..=(n - r) {
        res /= j;
    }
    println!("{}", res);
}
