use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }
    let mut a = 1;
    let mut b = 0;
    for _ in 0..n - 1 {
        let a2 = (x + 1) * a + b;
        let b2 = x * y * a + y * b;
        a = a2;
        b = b2;
    }
    println!("{}", b);
}
