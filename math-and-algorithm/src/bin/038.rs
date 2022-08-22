use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q],
    }
    let mut s = vec![0];
    for i in 0..n {
        s.push(s[i] + a[i]);
    }
    for (l, r) in &lr {
        println!("{}", s[*r] - s[*l - 1]);
    }
}
