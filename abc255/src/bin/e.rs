use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ss: [i64; n - 1],
        xs: [i64; m],
    }
    let mut ts = vec![0; n];
    for i in 1..n {
        ts[i] = -ts[i - 1] + ss[i - 1];
    }

    let mut map = HashMap::<_, usize>::new();
    for x in xs {
        for j in 0..n {
            let alpha = (if j % 2 == 0 { 1 } else { -1 }) * (x - ts[j]);
            *map.entry(alpha).or_default() += 1;
        }
    }
    let mut res = 0;
    for &val in map.values() {
        res = std::cmp::max(res, val);
    }
    println!("{}", res);
}
