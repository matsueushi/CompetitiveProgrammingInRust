use itertools::Itertools;
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        a: [Bytes; n],
    }
    let a = a
        .into_iter()
        .map(|x| x.into_iter().map(|x| x as u64 - '0' as u64).collect_vec())
        .collect_vec();

    let direction = vec![
        (1, 0),
        (1, 1),
        (0, 1),
        (n - 1, 1),
        (n - 1, 0),
        (n - 1, n - 1),
        (0, n - 1),
        (1, n - 1),
    ];

    let mut res = 0;
    for (dx, dy) in &direction {
        for i in 0..n {
            for j in 0..n {
                let mut current = 0;
                for k in 0..n {
                    current *= 10;
                    let x = (i + k * dx) % n;
                    let y = (j + k * dy) % n;
                    current += a[x][y];
                }
                res = std::cmp::max(res, current);
            }
        }
    }
    println!("{}", res);
}
