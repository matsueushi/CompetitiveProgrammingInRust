use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a1: usize,
        a2: usize,
        a3: usize,
    }
    println!("{}", a1 + a2 + a3);
}
