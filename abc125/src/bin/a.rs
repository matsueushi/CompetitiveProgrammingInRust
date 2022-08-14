use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        t: usize,
    }
    println!("{}", (t / a) * b);
}
