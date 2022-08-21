use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut res = 0.0;
    for i in 1..=n {
        res += 1.0 / (i as f64);
    }
    res *= n as f64;
    println!("{}", res);
}
