use itertools_num::ItertoolsNum;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        w: i64,
        stp: [(usize, usize, usize); n],
    }
    let t_max: usize = 200000;
    let mut bw = vec![0i64; t_max + 1];
    for (s, t, p) in &stp {
        bw[*s] += *p as i64;
        bw[*t] -= *p as i64;
    }
    let x: Vec<i64> = bw.iter().cumsum().collect();
    let mut res = 0;
    for v in x {
        res = res.max(v);
    }
    println!("{}", if res <= w { "Yes" } else { "No" });
}
