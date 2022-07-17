use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut u1 = (0..n).collect_vec();
    u1.sort_by_key(|&i| std::cmp::Reverse(&a[i]));

    let mut u2 = (0..n).collect_vec();
    u2.sort_by_key(|&i| std::cmp::Reverse(&b[i]));

    let mut u3 = (0..n).collect_vec();
    u3.sort_by_key(|&i| std::cmp::Reverse(&a[i] + &b[i]));

    let mut used = vec![false; n];

    let mut accs = vec![];
    for i in 0..x {
        accs.push(u1[i]);
        used[u1[i]] = true;
    }

    let mut ny = 0;
    for u in u2 {
        if ny == y {
            break;
        }
        if !used[u] {
            accs.push(u);
            ny += 1;
            used[u] = true;
        }
    }

    let mut nz = 0;
    for u in u3 {
        if nz == z {
            break;
        }
        if !used[u] {
            accs.push(u);
            nz += 1;
            used[u] = true;
        }
    }

    accs.sort();
    for u in accs {
        println!("{}", u + 1);
    }
}
