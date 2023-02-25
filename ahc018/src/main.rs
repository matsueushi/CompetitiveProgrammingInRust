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
    n: usize, // 土地のサイズ、 n = 200
    w: usize, // 水源の数、1 <= w <= 4
    k: usize, // 家の数、 1 <= k <= 10
    c: usize, // 体力の消費, c in 1,2,4,8,16,32,64,128
    source_pos: Vec<Pos>,
    house_pos: Vec<Pos>,
    field: Field,
    uf_node: UnionFind,
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
            uf_node: UnionFind::new(k + 1),
        }
    }

    pub fn all_connected(&mut self) -> bool {
        self.uf_node.group_size(self.k) == self.k + 1
    }

    pub fn connected(&mut self, i: usize) -> bool {
        self.uf_node.in_same_set(i, self.k)
    }

    pub fn solve(&mut self) {
        let mut connected_places = self.source_pos.clone();
        while !self.all_connected() {
            // 一番近いところを探す

            let mut from_idx = 0;
            let mut to_idx = 0;
            let mut dist = std::usize::MAX;
            // 接続されている場所を見ていく
            for i in 0..connected_places.len() {
                // 家を見ていく
                for j in 0..self.k {
                    // 繋がっている
                    if self.connected(j) {
                        continue;
                    }
                    let new_dist = connected_places[i].dist(&self.house_pos[j]);
                    if new_dist < dist {
                        dist = new_dist;
                        from_idx = i;
                        to_idx = j;
                    }
                }
            }

            self.mov(connected_places[from_idx], self.house_pos[to_idx]);
            self.uf_node.union(self.k, to_idx);
            connected_places.push(self.house_pos[to_idx]);
        }
    }

    pub fn mov_y(&mut self, start_y: usize, start_x: usize, goal_y: usize) {
        if start_y < goal_y {
            for y in start_y..=goal_y {
                self.destruct(y, start_x);
            }
        } else {
            for y in (goal_y..=start_y).rev() {
                self.destruct(y, start_x)
            }
        }
    }

    pub fn mov_x(&mut self, start_y: usize, start_x: usize, goal_x: usize) {
        if start_x < goal_x {
            for x in start_x..=goal_x {
                self.destruct(start_y, x);
            }
        } else {
            for x in (goal_x..=start_x).rev() {
                self.destruct(start_y, x);
            }
        }
    }

    pub fn mov(&mut self, start: Pos, goal: Pos) {
        println!(
            "# move from ({}, {}) to {} {}",
            start.y, start.x, goal.y, goal.x
        );
        self.mov_y(start.y, start.x, goal.y);
        self.mov_x(goal.y, start.x, goal.x);
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
