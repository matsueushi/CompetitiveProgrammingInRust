use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut l = 0.0;
    let mut r = 1000.0;
    for _ in 0..50 {
        let m = (l + r) / 2.0;
        if m * m * m + m < n as f64 {
            l = m;
        } else {
            r = m;
        }
    }
    println!("{}", l);
}
