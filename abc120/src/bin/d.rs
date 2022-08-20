use proconio::{fastout, input, marker::Usize1};

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

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut uf = UnionFind::new(n);
    let mut res = Vec::new();
    let mut score = n * (n - 1) / 2;
    for i in (0..m).rev() {
        let (a, b) = &ab[i];
        res.push(score);
        if uf.find_root(*a) != uf.find_root(*b) {
            score -= uf.group_size(*a) * uf.group_size(*b);
        }
        uf.union(*a, *b);
    }

    for x in res.iter().rev() {
        println!("{}", x);
    }
}
