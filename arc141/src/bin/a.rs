use proconio::{fastout, input};
use std::collections::BTreeSet;

fn solve(t: usize) -> usize {
    let mut m = t;
    let mut c = 0;
    while m > 0 {
        m /= 10;
        c += 1;
    }

    let mut ls = BTreeSet::new();
    for i in 2..=c {
        ls.insert(c / i);
    }

    let mut res = 10_usize.pow(c - 1) - 1;

    for l in ls {
        let mut a = 10_usize.pow(l - 1);
        let mut b = 10_usize.pow(l);
        while b - a > 1 {
            let m = (a + b) >> 1;
            let mut s = 0;
            for _ in 0..c / l {
                s *= 10_usize.pow(l);
                s += m;
            }
            if s <= t {
                a = m;
            } else {
                b = m;
            }
        }
        let mut sa = 0;
        for _ in 0..c / l {
            sa *= 10_usize.pow(l);
            sa += a;
        }
        if sa <= t {
            res = std::cmp::max(res, sa);
        }
    }
    res
}

#[fastout]
fn main() {
    input! {
        n: usize,
        ts: [usize; n],
    }

    for t in ts {
        let res = solve(t);
        println!("{}", res);
    }
}
