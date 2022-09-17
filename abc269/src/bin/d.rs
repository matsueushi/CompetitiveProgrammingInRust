use itertools::enumerate;
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut idx = HashMap::new();
    for (i, (x, y)) in enumerate(&xy) {
        idx.insert((*x, *y), i);
    }

    let mut uf = UnionFind::<usize>::new(n);

    let dx = [-1, -1, 0, 0, 1, 1];
    let dy = [-1, 0, -1, 1, 0, 1];
    for i in 0..n {
        let (x, y) = xy[i];
        for j in 0..6 {
            let (xx, yy) = (x + dx[j], y + dy[j]);
            if let Some(u) = idx.get(&(xx, yy)) {
                uf.union(i, *u);
            }
        }
    }

    println!(
        "{}",
        uf.into_labeling().into_iter().collect::<HashSet<_>>().len()
    );
}
