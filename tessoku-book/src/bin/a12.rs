use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut l = 1;
    let mut r = 1_000_000_000;
    while l < r {
        let mut p = 0;
        let m = (l + r) / 2;
        for ai in &a {
            p += m / ai;
        }
        if p >= k {
            r = m;
        } else {
            l = m + 1;
        }
    }
    println!("{}", r);
}
