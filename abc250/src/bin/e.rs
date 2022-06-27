use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        as_: [u64; n],
        bs: [u64; n],
        q: usize,
        xy: [(usize, usize); q],
    }
    let mut fs = vec![0; n + 1];
    let mut gs = vec![0; n + 1];

    let mut aset = HashSet::new();
    let mut j = 0;
    for i in 1..=n {
        aset.insert(as_[i - 1]);
        while j < n && aset.contains(&bs[j]) {
            j += 1;
        }
        fs[i] = j;
    }

    let mut bset = HashSet::new();
    j = 0;
    for i in 1..=n {
        bset.insert(bs[i - 1]);
        while j < n && bset.contains(&as_[j]) {
            j += 1;
        }
        gs[i] = j;
    }

    for (x, y) in xy {
        let res = if x <= gs[y] && y <= fs[x] {
            "Yes"
        } else {
            "No"
        };
        println!("{}", res);
    }
}
