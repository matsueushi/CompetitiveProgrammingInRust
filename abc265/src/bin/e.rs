use proconio::{fastout, input};
use std::collections::{HashMap, HashSet};

const P: usize = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        e: i64,
        f: i64,
        xy: [(i64, i64); m],
    }

    let obs: HashSet<_> = xy.into_iter().collect();
    let mut dp: HashMap<(i64, i64), usize> = HashMap::new();
    dp.insert((0, 0), 1);
    let pos = [(a, b), (c, d), (e, f)];
    for _ in 0..n {
        let mut dpnew = HashMap::new();
        for ((x, y), k) in dp {
            for (u, v) in &pos {
                let (wx, wy) = (x + u, y + v);
                if !obs.contains(&(wx, wy)) {
                    let v = dpnew.entry((wx, wy)).or_insert(0);
                    *v += k;
                    *v %= P;
                }
            }
        }
        dp = dpnew;
    }

    let mut res = 0;
    for (_, k) in &dp {
        res = (res + k) % P;
    }
    println!("{}", res);
}
