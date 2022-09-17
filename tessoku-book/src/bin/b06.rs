use std::cmp::Ordering;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize, usize); q],
    }
    let mut ok = vec![0];
    let mut ng = vec![0];
    for i in 0..n {
        ok.push(ok[i] + a[i]);
        ng.push(ng[i] + 1 - a[i]);
    }
    for (l, r) in &lr {
        let n_ok = ok[*r] - ok[*l - 1];
        let n_ng = ng[*r] - ng[*l - 1];
        let res = match n_ok.cmp(&n_ng) {
            Ordering::Less => "lose",
            Ordering::Equal => "draw",
            Ordering::Greater => "win",
        };
        println!("{}", res);
    }
}
