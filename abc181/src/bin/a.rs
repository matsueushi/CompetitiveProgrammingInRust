use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    println!("{}", if n % 2 == 0 { "White" } else { "Black" });
}
