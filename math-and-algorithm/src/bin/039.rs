use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        lrx: [(usize, usize, usize); q],
    }

    let mut hs: Vec<i64> = vec![0; n + 2];
    for (l, r, x) in &lrx {
        hs[*l] += *x as i64;
        hs[*r + 1] -= *x as i64;
    }

    let mut res = Vec::new();
    for i in 1..n {
        if hs[i + 1] < 0 {
            res.push('>');
        } else if hs[i + 1] == 0 {
            res.push('=');
        } else {
            res.push('<');
        }
    }
    println!("{}", res.into_iter().join(""));
}
