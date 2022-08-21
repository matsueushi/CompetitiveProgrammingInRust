use proconio::{fastout, input};

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    println!("{}", gcd(a, b));
}
