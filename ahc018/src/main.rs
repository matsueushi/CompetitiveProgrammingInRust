use proconio::{input, source::line::LineSource};
use std::io::BufReader;
use std::process;
use text_io::read;

#[derive(Debug, Clone)]
pub struct UnionFind {
    par: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    /// 大きさ `n` の Union-Find 木を初期化する。
    pub fn new(n: usize) -> Self {
        Self {
            par: vec![0; n],
            size: vec![1; n],
        }
    }

    /// 頂点 `a` の属する連結成分の代表元を返す。
    pub fn find_root(&mut self, a: usize) -> usize {
        if self.size[a] > 0 {
            return a;
        }
        self.par[a] = self.find_root(self.par[a]);
        self.par[a]
    }

    /// 辺 `(a, b)` を追加し、追加後の連結成分の代表元を返す。
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

    /// 頂点 `a` と 頂点 `b` が同じ連結成分に属しているかを返す。
    pub fn in_same_set(&mut self, a: usize, b: usize) -> bool {
        self.find_root(a) == self.find_root(b)
    }

    /// 頂点 `a` の属する連結成分のサイズを返す。
    pub fn group_size(&mut self, a: usize) -> usize {
        let x = self.find_root(a);
        self.size[x]
    }
}

#[derive(Clone, Copy)]
struct Pos {
    y: usize,
    x: usize,
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

impl Pos {
    pub fn dist(&self, other: &Pos) -> usize {
        abs_diff(self.x, other.x) + abs_diff(self.y, other.y)
    }
}

enum Response {
    NotBroken,
    Broken,
    Finish,
    InValid,
}

#[allow(unused)]
struct Field {
    n: usize,
    c: usize,
    is_broken: Vec<Vec<bool>>,
    total_cost: usize,
}

impl Field {
    pub fn new(n: usize, c: usize) -> Self {
        Self {
            n,
            c,
            is_broken: vec![vec![false; n]; n],
            total_cost: 0,
        }
    }

    pub fn query(&mut self, y: usize, x: usize, power: usize) -> Response {
        self.total_cost += power + self.c;
        println!("{} {} {}", y, x, power);

        let r: i32 = read!();
        // println!("# responce:{}", r);

        match r {
            0 => Response::NotBroken,
            1 => {
                self.is_broken[y][x] = true;
                Response::Broken
            }
            2 => {
                self.is_broken[y][x] = true;
                Response::Finish
            }
            _ => Response::InValid,
        }
    }
}

#[allow(unused)]
struct Solver {
    n: usize,
    w: usize,
    k: usize,
    c: usize,
    source_pos: Vec<Pos>,
    house_pos: Vec<Pos>,
    field: Field,
    uf: UnionFind,
}

impl Solver {
    pub fn new(
        n: usize,
        w: usize,
        k: usize,
        c: usize,
        source_pos: Vec<Pos>,
        house_pos: Vec<Pos>,
    ) -> Self {
        Self {
            n,
            w,
            k,
            c,
            source_pos,
            house_pos,
            field: Field::new(n, c),
            uf: UnionFind::new(k + 1),
        }
    }

    pub fn all_connected(&mut self) -> bool {
        self.uf.group_size(self.k) == self.k + 1
    }

    pub fn connected(&mut self, i: usize) -> bool {
        self.uf.in_same_set(i, self.k)
    }

    pub fn solve(&mut self) {
        while !self.all_connected() {
            for i in 0..self.k {
                // すでに水源に接続されている
                if self.connected(i) {
                    continue;
                }

                // 接続されていない
                let mut hidx = 0;
                let mut hdist = std::usize::MAX;
                for j in 0..self.k {
                    if self.uf.in_same_set(i, j) {
                        continue;
                    }
                    let d = self.house_pos[i].dist(&self.house_pos[j]);
                    if d < hdist {
                        hdist = d;
                        hidx = j;
                    }
                }

                let mut sidx = 0;
                let mut sdist = std::usize::MAX;
                for j in 0..self.w {
                    let d = self.house_pos[i].dist(&self.source_pos[j]);
                    if d < sdist {
                        sdist = d;
                        sidx = j;
                    }
                }

                if sdist < hdist {
                    // 水源に繋いだ方が近い
                    self.mov(self.house_pos[i], self.source_pos[sidx]);
                    self.uf.union(i, self.k);
                } else {
                    self.mov(self.house_pos[i], self.house_pos[hidx]);
                    self.uf.union(i, hidx);
                }
            }
        }
    }

    pub fn mov(&mut self, start: Pos, goal: Pos) {
        println!(
            "# move from ({}, {}) to {} {}",
            start.y, start.x, goal.y, goal.x
        );

        if start.y < goal.y {
            for y in start.y..=goal.y {
                self.destruct(y, start.x);
            }
        } else {
            for y in (goal.y..=start.y).rev() {
                self.destruct(y, start.x)
            }
        }

        if start.x < goal.x {
            for x in start.x..=goal.x {
                self.destruct(goal.y, x);
            }
        } else {
            for x in (goal.x..=start.x).rev() {
                self.destruct(goal.y, x);
            }
        }
    }

    pub fn destruct(&mut self, y: usize, x: usize) {
        const POWER: usize = 100;
        while !self.field.is_broken[y][x] {
            let result = self.field.query(y, x, POWER);
            match result {
                Response::Finish => {
                    // eprintln!("total_cost={}", self.field.total_cost);
                    process::exit(0);
                }
                Response::InValid => {
                    // eprintln!("invalid: y={}, x={}", y, x);
                    process::exit(1);
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut source = LineSource::new(BufReader::new(stdin));

    input! {
        from &mut source,
        n: usize,
        w: usize,
        k: usize,
        c: usize,
        ab: [(usize, usize); w],
        cd: [(usize, usize); k],
    }

    let mut source_pos = Vec::new();
    let mut house_pos = Vec::new();
    for (a, b) in ab {
        source_pos.push(Pos { y: a, x: b });
    }
    for (c, d) in cd {
        house_pos.push(Pos { y: c, x: d });
    }

    let mut solver = Solver::new(n, w, k, c, source_pos, house_pos);
    solver.solve();
}

// ../target/release/tester cargo run < tools/in/0000.txt > tools/out/0000.txt
