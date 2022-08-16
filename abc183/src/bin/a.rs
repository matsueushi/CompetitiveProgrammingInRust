use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i64,
    }
    println!("{}", 0.max(x));
}
