use proconio::{fastout, input};
use std::cmp::Ordering::{Greater, Less};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        a: i64,
        mut xh: [(usize, i64); n],
    }
    xh.sort();
    let mut xs = Vec::new();
    let mut hs = Vec::new();
    for (x, h) in xh {
        xs.push(x);
        hs.push(h);
    }

    let mut x2ds = Vec::new();
    for x in &xs {
        let i = xs
            .binary_search_by(|&v| if v <= x + 2 * d { Less } else { Greater })
            .unwrap_or_else(|i| i);
        x2ds.push(i);
    }

    let mut dtable = vec![0; n + 1];
    let mut dmg: i64 = 0;
    let mut res = 0;
    for i in 0..n {
        dmg += dtable[i];
        if hs[i] <= dmg {
            continue;
        }
        let atk = num::Integer::div_ceil(&(hs[i] - dmg), &a);
        res += atk;
        let dmgi = a * atk;
        dmg += dmgi;
        dtable[x2ds[i]] -= dmgi;
    }

    println!("{}", res);
}
