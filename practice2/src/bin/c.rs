use floor_sum::floor_sum;
use proconio::{fastout, input};

pub mod floor_sum {
    pub fn floor_sum(n: usize, m: usize, a: usize, b: usize) -> usize {
        let (x, a0) = (a / m, a % m);
        let (y, b0) = (b / m, b % m);
        let s = if n <= 1 { 0 } else { n * (n - 1) / 2 * x } + n * y;
        if a0 == 0 {
            return s;
        }
        let u = a0 * n + b0;
        let (c, d) = (u / m, u % m);
        s + floor_sum(c, a0, m, d)
    }
}

#[fastout]
fn main() {
    input! {
        t: usize,
        nmab: [(usize, usize, usize, usize); t],
    }

    for (n, m, a, b) in nmab {
        println!("{}", floor_sum(n, m, a, b));
    }
}
