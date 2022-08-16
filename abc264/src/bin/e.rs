use proconio::{fastout, input};
use std::collections::HashSet;

pub mod union_find {

    #[derive(Debug, Clone)]
    pub struct UnionFind {
        par: Vec<usize>,
        size: Vec<usize>,
    }

    impl UnionFind {
        pub fn new(n: usize) -> Self {
            Self {
                par: vec![0; n],
                size: vec![1; n],
            }
        }

        pub fn find_root(&mut self, a: usize) -> usize {
            if self.size[a] > 0 {
                return a;
            }
            self.par[a] = self.find_root(self.par[a]);
            self.par[a]
        }

        pub fn union(&mut self, a: usize, b: usize) -> usize {
            let mut x = self.find_root(a);
            let mut y = self.find_root(b);
            if x == y {
                return x;
            }
            if self.size[x] < self.size[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.size[x] += self.size[y];
            self.size[y] = 0;
            self.par[y] = x;
            x
        }

        pub fn in_same_set(&mut self, a: usize, b: usize) -> bool {
            self.find_root(a) == self.find_root(b)
        }

        pub fn group_size(&mut self, a: usize) -> usize {
            let x = self.find_root(a);
            self.size[x]
        }
    }
}

#[fastout]
fn main() {
    use union_find::UnionFind;

    input! {
        n: usize,
        m: usize,
        e: usize,
        uv: [(usize, usize); e],
        q: usize,
        x: [usize; q],
    }
    let mut uv2 = Vec::new();
    for (u, v) in uv {
        let x = if u > n { 0 } else { u };
        let y = if v > n { 0 } else { v };
        uv2.push((x, y))
    }

    // 0: plant
    let mut uf = UnionFind::new(n + 1);

    let xset: HashSet<_> = x.iter().cloned().collect();
    for i in 0..e {
        if xset.contains(&(i + 1)) {
            continue;
        }
        let (x, y) = uv2[i];
        uf.union(x, y);
    }
    let mut res = Vec::new();
    for i in x.iter().rev() {
        res.push(uf.group_size(0) - 1);
        let (x, y) = uv2[*i - 1];
        uf.union(x, y);
    }

    for v in res.iter().rev() {
        println!("{}", v);
    }
}
