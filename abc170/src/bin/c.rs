use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i64,
        n: usize,
        ps: [usize; n],
    }
    let pss: HashSet<_> = ps.into_iter().collect();
    let mut res = 0;
    let mut a = std::i64::MAX;
    for i in 0..=101 {
        if pss.contains(&i) {
            continue;
        }
        let c = (x - i as i64).abs();
        if c < a {
            a = c;
            res = i;
        }
    }
    println!("{}", res);
}
