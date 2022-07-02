use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut uf = UnionFind::new(n);
    let mut res = vec![];

    for _ in 0..q {
        input! {
            t: usize,
            u: usize,
            v: usize,
        }
        match t {
            0 => {
                uf.union(u, v);
            }
            1 => {
                let x = if uf.equiv(u, v) { 1 } else { 0 };
                res.push(x);
            }
            _ => unreachable!(),
        }
    }

    for x in res {
        println!("{}", x);
    }
}
