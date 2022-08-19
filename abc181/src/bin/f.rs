use proconio::{fastout, input};
use union_find::*;

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

fn f(xy: &Vec<(i64, i64)>, n: usize, r: f64) -> bool {
    // n: y=-100, n+1: y=100
    let mut uf = UnionFind::new(xy.len() + 2);
    for i in 0..n {
        let (_, y) = xy[i];
        let y = y as f64;
        if y - 2.0 * r < -100.0 {
            uf.union(i, n);
        }
        if y + 2.0 * r > 100.0 {
            uf.union(i, n + 1);
        }
    }

    for i in 0..n - 1 {
        let (x0, y0) = xy[i];
        let x0 = x0 as f64;
        let y0 = y0 as f64;
        for j in i + 1..n {
            let (x1, y1) = xy[j];
            let x1 = x1 as f64;
            let y1 = y1 as f64;
            let dx = x1 - x0;
            let dy = y1 - y0;
            if dx * dx + dy * dy < 4.0 * r * r {
                uf.union(i, j);
            }
        }
    }

    !uf.in_same_set(n, n + 1)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    // bisect
    let mut l = 0.0;
    let mut r = 100.0;
    for _ in 0..100 {
        let mid = (l + r) / 2.0;
        if f(&xy, n, mid) {
            l = mid;
        } else {
            r = mid;
        }
    }

    println!("{}", r);
}
