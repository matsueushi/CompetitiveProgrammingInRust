use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: usize,
    }
    let mut res = 1;
    for k in 1..=11 {
        res *= l - k;
        res /= k;
    }
    println!("{}", res);
}
