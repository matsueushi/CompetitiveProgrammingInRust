use proconio::{fastout, input};

const P: i64 = 998244353;

#[fastout]
fn main() {
    input! {
        n: i64,
    }
    println!("{}", (P + n % P) % P);
}
