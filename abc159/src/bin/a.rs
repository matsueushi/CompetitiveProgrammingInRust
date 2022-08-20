use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
        m: i64,
    }
    let res = n * (n - 1) / 2 + m * (m - 1) / 2;
    println!("{}", res);
}
