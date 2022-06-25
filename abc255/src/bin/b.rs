use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        k: usize,
        as_: [usize; k],
        xy: [(i64, i64); n],
    }
    let as_: Vec<usize> = as_.iter().map(|x| x - 1).collect();

    let mut d = 0;
    for (x, y) in &xy {
        let mut di = std::i64::MAX;
        for &a in &as_ {
            let (x2, y2) = xy[a];
            di = cmp::min(di, (x - x2).pow(2) + (y - y2).pow(2));
        }
        d = cmp::max(d, di);
    }
    println!("{}", (d as f64).sqrt());
}
