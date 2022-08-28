use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        a: usize,
    }
    println!("{}", num::Integer::div_ceil(&h, &a));
}
