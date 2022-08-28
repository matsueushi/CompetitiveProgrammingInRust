use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n],
    }
    h.sort();
    let mut res = 0;
    let p = k.min(n);
    for i in 0..n - p {
        res += h[i];
    }
    println!("{}", res);
}
