use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut res = 0;
    for i in 1..=n {
        let m = n / i;
        res += i * (m * (m + 1)) / 2;
    }
    println!("{}", res);
}
