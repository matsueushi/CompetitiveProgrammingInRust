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

impl Default for Pos {
    fn default() -> Self {
        Self { y: 0, x: 0 }
    }
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
    total_cost: usize,
}

impl Field {
    pub fn new(n: usize, c: usize) -> Self {
        Self {
            n,
            c,
            is_broken: vec![vec![false; n]; n],
            cost: vec![vec![0; n]; n],
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
    connected: Vec<bool>, // 接続されているかを確認する
    n_connected: usize,
    uf_field: UnionFind<usize>, // フィールドの連結状況を管理する
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
        let mut uf_field = UnionFind::new(n * n + 1);
        // ソースを接続しておく
        for source in &source_pos {
            uf_field.union(source.y * n + source.x, n * n);
        }
        Self {
            n,
            w,
            k,
            c,
            source_pos,
            house_pos,
            field: Field::new(n, c),
            connected: vec![false; n],
            n_connected: 0,
            uf_field: uf_field, // n * n  はソース
        }
    }

    pub fn all_connected(&mut self) -> bool {
        self.n_connected == self.n
    }

    pub fn field_index(&self, y: usize, x: usize) -> usize {
        y * self.n + x
    }

    pub fn is_reachable_pos(&mut self, y: usize, x: usize) -> bool {
        self.uf_field.equiv(self.field_index(y, x), self.n * self.n)
    }

    pub fn update_graph(&mut self, start_y: usize, start_x: usize, goal_y: usize, goal_x: usize) {
        // アップデートする
        self.uf_field.union(
            self.field_index(start_y, start_x),
            self.field_index(goal_y, goal_x),
        );
    }

    pub fn find_nearest(&mut self, pos: Pos) -> Pos {
        // 一番近い場所を探す
        let y = pos.y as i32;
        let x = pos.x as i32;
        for d in 1..2 * self.n as i32 {
            for nx in x - d..=x + d {
                if nx < 0 || nx >= self.n as i32 {
                    continue;
                }
                let c = d - (nx - x).abs();
                for ny in [y - c, y + c] {
                    if ny < 0 || ny >= self.n as i32 {
                        continue;
                    }

                    // println!("# a {} {} {} {}", y, x, ny, nx);
                    if ny == 104 && nx == 45 {
                        println!("# okok");
                    }

                    if self.is_reachable_pos(ny as usize, nx as usize) {
                        // println!("# a {} {} {} {}", y, x, ny, nx);
                        return Pos {
                            y: ny as usize,
                            x: nx as usize,
                        };
                    }
                }
            }
        }
        pos
    }

    pub fn solve(&mut self) {
        while !self.all_connected() {
            let mut i_start = 0;
            let mut goal = Pos::default();
            let mut dist = std::usize::MAX;

            // 家を見ていく
            for i in 0..self.k {
                if self.connected[i] {
                    continue;
                }
                let new_goal = self.find_nearest(self.house_pos[i]);
                let new_dist = self.house_pos[i].dist(&new_goal);
                println!(
                    "# new goal {} {} {} {} new dist {}",
                    self.house_pos[i].y, self.house_pos[i].x, new_goal.y, new_goal.x, new_dist
                );
                if new_dist < dist {
                    dist = new_dist;
                    i_start = i;
                    goal = new_goal;
                }
            }

            println!("# !!! {} {} {}", i_start, goal.y, goal.x);
            self.mov(self.house_pos[i_start], goal);
            self.connected[i_start] = true;
            self.n_connected += 1;
        }
    }

    pub fn mov(&mut self, start: Pos, goal: Pos) {
        println!(
            "# move from ({}, {}) to {} {}",
            start.y, start.x, goal.y, goal.x
        );
        let mut hist = Vec::new();
        let mut cur_y = start.y;
        let mut cur_x = start.x;
        self.destruct(cur_y, cur_x);
        self.update_graph(start.y, start.x, cur_y, cur_x);
        hist.push((cur_y, cur_x));
        while cur_y != goal.y || cur_x != goal.x {
            let dy = abs_diff(cur_y, goal.y);
            let dx = abs_diff(cur_x, goal.x);
            if dx == 0 {
                // y 方向に動かす
                toward(&mut cur_y, goal.y);
            } else if dy == 0 {
                // x 方向に動かす
                toward(&mut cur_x, goal.x);
            } else {
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
                        if cur_y != prev_y {
                            toward(&mut cur_x, goal.x);
                        } else {
                            toward(&mut cur_y, goal.y);
                        }
                    } else {
                        if cur_y != prev_y {
                            toward(&mut cur_y, goal.y);
                        } else {
                            toward(&mut cur_x, goal.x);
                        }
                    }
                }
            }
            // コストを計算
            self.destruct(cur_y, cur_x);
            // グラフを更新
            self.update_graph(start.y, start.x, cur_y, cur_x);
            // 履歴を更新
            hist.push((cur_y, cur_x));
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
