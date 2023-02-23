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
    c: usize,
    source_pos: Vec<Pos>,
    house_pos: Vec<Pos>,
    field: Field,
}

impl Solver {
    pub fn new(n: usize, source_pos: Vec<Pos>, house_pos: Vec<Pos>, c: usize) -> Self {
        Self {
            n,
            c,
            source_pos,
            house_pos,
            field: Field::new(n, c),
        }
    }

    pub fn solve(&mut self) {
        let hl = self.house_pos.len();
        let mut uf = UnionFind::new(hl + 1);

        while uf.group_size(hl) != hl + 1 {
            for i in 0..hl {
                // すでに水源に接続されている
                if uf.in_same_set(i, hl) {
                    continue;
                }

                // 接続されていない
                let mut hidx = 0;
                let mut hdist = std::usize::MAX;
                for j in 0..hl {
                    if uf.in_same_set(i, j) {
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
                for j in 0..self.source_pos.len() {
                    let d = self.house_pos[i].dist(&self.source_pos[j]);
                    if d < sdist {
                        sdist = d;
                        sidx = j;
                    }
                }

                if sdist < hdist {
                    // 水源に繋いだ方が近い
                    self.mov(self.house_pos[i], self.source_pos[sidx]);
                    uf.union(i, hl);
                } else {
                    self.mov(self.house_pos[i], self.house_pos[hidx]);
                    uf.union(i, hidx);
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

    pub fn destruct(&mut self, y: usize, x: usize) -> bool {
        if self.field.is_broken[y][x] {
            // すでに壊れていたらスキップ
            return false;
        }
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
        true
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

    let mut solver = Solver::new(n, source_pos, house_pos, c);
    solver.solve();
}

// ../target/release/tester cargo run < tools/in/0000.txt > tools/out/0000.txt
