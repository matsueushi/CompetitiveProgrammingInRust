use proconio::{fastout, input, marker::Usize1};
use std::collections::HashMap;

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
        q: usize,
        c: [Usize1; n],
        query: [(usize, Usize1, Usize1); q],
    }

    let mut uf = UnionFind::new(n);
    let mut group = Vec::new();
    for i in 0..n {
        let mut hashmap = HashMap::new();
        hashmap.insert(c[i], 1_usize);
        group.push(hashmap);
    }

    for i in 0..q {
        match query[i] {
            (1, a, b) => {
                let a = uf.find_root(a);
                let b = uf.find_root(b);
                if a == b {
                    continue;
                }
                let par = uf.union(a, b);
                let (u, v) = if par == a { (a, b) } else { (b, a) };
                let t = group[v].clone();
                for (key, val) in t {
                    *group[u].entry(key).or_insert(0) += val;
                }
            }
            (2, x, y) => {
                let par = uf.find_root(x);
                let np = group[par].get(&y);
                match np {
                    Some(m) => {
                        println!("{}", m);
                    }
                    None => {
                        println!("0");
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
