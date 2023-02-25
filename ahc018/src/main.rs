use petgraph::unionfind::UnionFind;
use proconio::{input, source::line::LineSource};
use std::io::BufReader;
use std::process;
use text_io::read;

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
    cost: Vec<Vec<i64>>,
    n_trial: Vec<Vec<usize>>,
    total_cost: usize,
}

impl Field {
    pub fn new(n: usize, c: usize) -> Self {
        Self {
            n,
            c,
            is_broken: vec![vec![false; n]; n],
            cost: vec![vec![0; n]; n],
            n_trial: vec![vec![0; n]; n],
            total_cost: 0,
        }
    }

    pub fn query(&mut self, y: usize, x: usize, power: usize) -> Response {
        self.total_cost += power + self.c;
        self.cost[y][x] += (power + self.c) as i64;
        println!("{} {} {}", y, x, power);

        let r: i32 = read!();

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
    uf_node: UnionFind<usize>,
    n_connected: usize,
    base_power: usize,
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
            n_connected: 0,
            base_power: 100,
        }
    }

    pub fn all_connected(&mut self) -> bool {
        self.n_connected == self.k
    }

    pub fn connected(&mut self, i: usize) -> bool {
        self.uf_node.equiv(i, self.k)
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
            self.n_connected += 1;
        }
    }

    pub fn update_power(&mut self, cur_y: usize, cur_x: usize) {
        // あまり効果は見られない。
        // 力の調節よりも、場所の探索の方が重要そう。
        let n_trial = self.field.n_trial[cur_y][cur_x] as i64;

        if n_trial > 1 {
            self.base_power = 100;
        } else {
            self.base_power = ((self.base_power as f64) * 0.9).max(25.0).round() as usize;
        }
    }

    pub fn mov(&mut self, start: Pos, goal: Pos) {
        println!(
            "# move from ({}, {}) to {} {}",
            start.y, start.x, goal.y, goal.x
        );
        // スタートからゴールまでの角度
        let mut hist = Vec::new();
        let mut cur_y = start.y;
        let mut cur_x = start.x;
        self.destruct(cur_y, cur_x);
        hist.push((cur_y, cur_x));
        while cur_y != goal.y || cur_x != goal.x {
            let dy = abs_diff(cur_y, goal.y);
            let dx = abs_diff(cur_x, goal.x);

            self.update_power(cur_y, cur_x);

            let len = hist.len();
            if len == 1 {
                if dy > dx {
                    toward(&mut cur_y, goal.y);
                } else {
                    toward(&mut cur_x, goal.x);
                }
            } else {
                // ここで勾配を計算したい
                let (prev_y, prev_x) = hist[len - 2];
                let grad = self.field.cost[cur_y][cur_x] - self.field.cost[prev_y][prev_x];

                if grad > 0 {
                    // 勾配が正なので、避けていけないか
                    if (dx != 0 && cur_y != prev_y) || dy == 0 {
                        toward(&mut cur_x, goal.x);
                    } else {
                        toward(&mut cur_y, goal.y);
                    }
                } else {
                    if (dy != 0 && cur_y != prev_y) || dx == 0 {
                        // 同じ方向に進む
                        toward(&mut cur_y, goal.y);
                    } else {
                        toward(&mut cur_x, goal.x);
                    }
                }
            }
            // コストを計算
            self.destruct(cur_y, cur_x);
            hist.push((cur_y, cur_x));
        }
    }

    pub fn destruct(&mut self, y: usize, x: usize) {
        while !self.field.is_broken[y][x] {
            self.field.n_trial[y][x] += 1;
            let result = self.field.query(y, x, self.base_power);
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

pub fn toward(v: &mut usize, nv: usize) {
    if *v < nv {
        *v += 1;
    } else {
        *v -= 1;
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
