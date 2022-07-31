use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }

    let edges: HashSet<_> = uv.into_iter().collect();

    let mut res = 0;
    for a in 1..=n {
        for b in a + 1..=n {
            for c in b + 1..=n {
                if edges.contains(&(a, b)) && edges.contains(&(b, c)) && edges.contains(&(a, c)) {
                    res += 1;
                }
            }
        }
    }
    println!("{}", res);
}
