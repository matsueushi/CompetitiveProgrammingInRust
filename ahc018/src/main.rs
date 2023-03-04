#![allow(dead_code)]
#![allow(unused)]

use proconio::{input, source::line::LineSource};
use std::io::BufReader;
use std::process;
#[cfg(feature = "local")]
use svg::node::element::{path::Data, Path};
use text_io::read;

const INF: usize = std::usize::MAX / 2;

#[cfg(feature = "local")]
fn create_rect_data(y0: usize, x0: usize, y1: usize, x1: usize) -> Data {
    Data::new()
        .move_to((x0, y0))
        .line_by((x1 as i64 - x0 as i64, 0))
        .line_by((0, y1 as i64 - y0 as i64))
        .line_by((x0 as i64 - x1 as i64, 0))
        .close()
}

#[cfg(feature = "local")]
fn create_line(x1: i64, y1: i64, x2: i64, y2: i64) -> svg::node::element::Line {
    svg::node::element::Line::new()
        .set("x1", x1 as usize)
        .set("y1", y1 as usize)
        .set("x2", x2 as usize)
        .set("y2", y2 as usize)
        .set("stroke", "black")
        .set("stroke-width", 0.5)
}

#[cfg(feature = "local")]
fn create_circ(x: i64, y: i64, r: usize, stroke: &str) -> svg::node::element::Circle {
    svg::node::element::Circle::new()
        .set("cx", x as usize)
        .set("cy", y as usize)
        .set("r", r)
        .set("stroke", stroke)
        .set("stroke-width", 1)
        .set("fill", "transparent")
}

#[cfg(feature = "local")]
fn create_text(
    x: i64,
    y: i64,
    font_size: usize,
    text: &str,
    fill: &str,
) -> svg::node::element::Text {
    svg::node::element::Text::new()
        .set("x", x)
        .set("y", y)
        .set("font-size", font_size)
        .set("font_family", "sans")
        .set("fill", fill)
        .add(svg::node::Text::new(text))
}

/// 場所を示す構造体。
#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct Pos {
    y: i64,
    x: i64,
}

impl Pos {
    /// マンハッタン距離
    pub fn manhattan_dist(&self, other: &Self) -> i64 {
        (self.y - other.y).abs() + (self.x - other.x).abs()
    }
}

/// テスターから帰ってくるレスポンス
enum Response {
    NotBroken,
    Broken,
    Finish,
    InValid,
}

/// フィールド
struct Field {
    n: usize,
    w: usize,
    k: usize,
    source_pos: Vec<Pos>,
    house_pos: Vec<Pos>,
    power: Vec<Vec<usize>>,    // 掘削するのに消費したパワー
    n_trial: Vec<Vec<usize>>,  // 掘削した回数
    is_broken: Vec<Vec<bool>>, // すでにフィールドが壊れているか
    conn: Vec<usize>,
}

impl Field {
    pub fn new(n: usize, source_pos: Vec<Pos>, house_pos: Vec<Pos>) -> Self {
        let k = house_pos.len();
        Self {
            n,
            w: source_pos.len(),
            k,
            source_pos,
            house_pos,
            power: vec![vec![0; n]; n],
            n_trial: vec![vec![0; n]; n],
            is_broken: vec![vec![false; n]; n],
            conn: vec![0; k],
        }
    }

    /// 破壊クエリを実施する
    pub fn query(&mut self, pos: &Pos, power: usize) -> Response {
        let (y, x) = (pos.y as usize, pos.x as usize);
        self.power[y][x] += power;
        self.n_trial[y][x] += 1;

        // クエリ発行
        println!("{} {} {}", pos.y, pos.x, power);

        // 結果の取得
        match read!() {
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

    pub fn dist<F>(&self, dist_fun: F) -> Vec<Vec<usize>>
    where
        F: Fn(Pos, Pos) -> i64,
    {
        let mut dist = vec![vec![INF; self.w + self.k]; self.w + self.k];
        // 水源どうしの場所は、距離を0にする
        for i in 0..self.w {
            for j in 0..=i {
                dist[i][j] = 0;
                dist[j][i] = 0;
            }
        }
        dist[self.k][self.k] = 0;
        for i in 0..self.k {
            // 家同士の距離
            for j in 0..=i {
                let d = dist_fun(self.house_pos[i], self.house_pos[j]);
                let wi = self.w + i;
                let wj = self.w + j;
                dist[wi][wj] = d as usize;
                dist[wj][wi] = d as usize;
            }
            // 水源と家との距離
            for l in 0..self.w {
                let d = dist_fun(self.house_pos[i], self.source_pos[l]);
                dist[self.w + i][l] = d as usize;
                dist[l][self.w + i] = d as usize;
            }
        }
        dist
    }

    pub fn prim(&mut self, dist: Vec<Vec<usize>>) {
        // Prim法
        let mut min_cost = vec![INF; self.w + self.k];
        let mut min_node = vec![INF; self.w + self.k];
        let mut used = vec![false; self.w + self.k];

        // eprintln!("{:?}", dist);

        for i in 0..self.w {
            min_cost[i] = 0;
            min_node[i] = i;
            used[i] = true;
            for j in 0..self.k {
                let wj = self.w + j;
                if dist[i][wj] < min_cost[wj] {
                    min_cost[wj] = dist[i][wj];
                    min_node[wj] = i;
                }
            }
        }

        loop {
            // X に属さない頂点からコストが最小になる点を探す
            let mut v = None;
            for u in 0..self.w + self.k {
                if used[u] {
                    continue;
                }
                if let Some(i) = v {
                    if min_cost[u] < min_cost[i] {
                        v = Some(u);
                    }
                } else {
                    v = Some(u);
                }
            }

            // eprintln!("{:?}", v);

            // コストが最小になる点を追加し、距離を更新する
            if let Some(i) = v {
                used[i] = true;
                if i >= self.w {
                    self.conn[i - self.w] = min_node[i];
                }

                for u in 0..self.w + self.k {
                    // eprintln!("{} {} {} {}", i, u, dist[i][u], min_cost[u]);
                    if dist[u][i] < min_cost[u] {
                        min_cost[u] = dist[u][i];
                        min_node[u] = i;
                    }
                }
            } else {
                break;
            }
            // eprintln!("{:?}", min_cost);
            // eprintln!("{:?}", min_node);
            // eprintln!("{:?}", self.conn);
        }
    }

    #[cfg(not(feature = "local"))]
    pub fn save_svg(&self) {}

    #[cfg(feature = "local")]
    pub fn save_svg(&self) {
        const MARGIN: usize = 20;

        let w = self.n;
        let d = w + 2 * MARGIN;
        let mut doc = svg::Document::new().set("viewBox", (0, 0, d, d));
        let back = Path::new()
            .set("fill", "white")
            .set("d", create_rect_data(0, 0, w, w));
        doc = doc.add(back);
        for i in 0..self.w {
            let pos = self.source_pos[i];
            let circ = create_circ(pos.x, pos.y, 5, "blue");
            doc = doc.add(circ);
            let text = create_text(pos.x + 5, pos.y + 5, 8, &i.to_string(), "blue");
            doc = doc.add(text);
        }
        for i in 0..self.k {
            let pos = self.house_pos[i];
            let circ = create_circ(pos.x, pos.y, 5, "green");
            doc = doc.add(circ);
            let text = create_text(pos.x + 5, pos.y + 5, 8, &(self.w + i).to_string(), "green");
            doc = doc.add(text);
        }
        for i in 0..self.k {
            let start = &self.house_pos[i];
            let goal = if self.conn[i] < self.w {
                self.source_pos[self.conn[i]]
            } else {
                self.house_pos[self.conn[i] - self.w]
            };
            // 線を追加
            let line = create_line(start.x, start.y, goal.x, goal.y);
            doc = doc.add(line);
            // 距離を追加
            let d = start.manhattan_dist(&goal);
            let text = create_text(
                (start.x + goal.x) / 2,
                (start.y + goal.y) / 2,
                8,
                &d.to_string(),
                "black",
            );
            doc = doc.add(text);
        }
        svg::save("field.svg", &doc).unwrap();
    }
}

/// ボーリング
struct BoringResult {
    n: usize,
    d: usize,
    ny: usize,
    nx: usize,
    ys: Vec<i64>,
    xs: Vec<i64>,
    sturdiness: Vec<Vec<usize>>,
    dist: Vec<Vec<usize>>,
    next: Vec<Vec<usize>>, // i から j への最短経路における i の一つ後の点
}

impl BoringResult {
    pub fn new(n: usize, d: usize) -> Self {
        let mut ys: Vec<_> = (0_i64..n as i64).step_by(d).collect();
        if *ys.last().unwrap() != (n - 1) as i64 {
            ys.push((n - 1) as i64);
        }
        let mut xs: Vec<_> = (0_i64..n as i64).step_by(d).collect();
        if *xs.last().unwrap() != (n - 1) as i64 {
            xs.push((n - 1) as i64);
        }
        let ny = ys.len();
        let nx = xs.len();
        let sturdiness = vec![vec![INF; ys.len()]; xs.len()];
        let dist = vec![vec![INF; ny * nx]; ny * nx];
        let next = vec![vec![INF; ny * nx]; ny * nx];
        Self {
            n,
            d,
            ny,
            nx,
            ys,
            xs,
            sturdiness,
            dist,
            next,
        }
    }

    pub fn calculate_dist(&mut self) {
        let nn = self.ny * self.nx;
        for i in 0..self.ny {
            for j in 0..self.nx {
                let c = i * self.ny + j;
                self.dist[c][c] = 0;
                if i != self.ny - 1 {
                    self.dist[c][c + 1 * self.ny] =
                        self.sturdiness[i][j] + self.sturdiness[i + 1][j];
                    self.dist[c + 1 * self.ny][c] = self.dist[c][c + 1 * self.ny];
                }
                if j != self.nx - 1 {
                    self.dist[c][c + 1] = self.sturdiness[i][j] + self.sturdiness[i][j + 1];
                    self.dist[c + 1][c] = self.dist[c][c + 1];
                }
            }
        }

        // ワーシャルフロイド
        for i in 0..nn {
            for j in 0..nn {
                self.next[i][j] = j;
            }
        }

        for k in 0..nn {
            for i in 0..nn {
                for j in 0..nn {
                    let d = self.dist[i][k] + self.dist[k][j];
                    if d < self.dist[i][j] {
                        self.dist[i][j] = d;
                        self.next[i][j] = self.next[i][k];
                    }
                }
            }
        }
    }

    pub fn near_best_idx(&self, pos: Pos) -> (usize, usize) {
        let (mut bi, mut bj) = (0, 0);
        let mut s = INF;
        for i in 0..self.ny {
            for j in 0..self.nx {
                if (pos.y - self.ys[i]).abs() <= self.d as i64
                    && (pos.x - self.xs[j]).abs() <= self.d as i64
                    && self.sturdiness[i][j] < s
                {
                    s = self.sturdiness[i][j];
                    (bi, bj) = (i, j);
                }
            }
        }
        (bi, bj)
    }

    pub fn idx_to_pos(&self, i: usize, j: usize) -> Pos {
        Pos {
            y: self.ys[i],
            x: self.xs[j],
        }
    }

    pub fn dist(&self, start: Pos, goal: Pos) -> i64 {
        let (si, sj) = self.near_best_idx(start);
        let (gi, gj) = self.near_best_idx(goal);
        let ds = self.sturdiness[si][sj] as i64
            * start.manhattan_dist(&Pos {
                y: self.ys[si],
                x: self.xs[sj],
            });
        let dg = self.sturdiness[gi][gj] as i64
            * start.manhattan_dist(&Pos {
                y: self.ys[gi],
                x: self.xs[gj],
            });
        ds + (self.d * self.dist[si * self.ny + sj][gi * self.ny + gj]) as i64 + dg
    }

    pub fn shortest_path(&self, start_idx: (usize, usize), goal_idx: (usize, usize)) -> Vec<Pos> {
        let mut start_idx = start_idx;
        let mut path = Vec::new();
        path.push(self.idx_to_pos(start_idx.0, start_idx.1));
        while start_idx != goal_idx {
            let next =
                self.next[start_idx.0 * self.ny + start_idx.1][goal_idx.0 * self.ny + goal_idx.1];
            start_idx = (next / self.ny, next % self.ny);
            path.push(self.idx_to_pos(start_idx.0, start_idx.1));
        }
        path
    }

    #[cfg(not(feature = "local"))]
    pub fn save_svg(&self) {}

    #[cfg(feature = "local")]
    pub fn save_svg(&self) {
        const MARGIN: i64 = 20;

        let w = self.n;
        let d = w as i64 + 2 * MARGIN;
        let mut doc = svg::Document::new().set("viewBox", (0, 0, d, d));

        for i in 0..self.ny {
            for j in 0..self.nx {
                let text = create_text(
                    self.xs[j] + MARGIN,
                    self.ys[i] + MARGIN,
                    4,
                    &self.sturdiness[i][j].to_string(),
                    "black",
                );
                doc = doc.add(text);

                if i != self.ny - 1 {
                    let x = self.xs[j] + MARGIN;
                    let y = (self.ys[i] + self.ys[i + 1]) / 2 + MARGIN;
                    let s = (self.sturdiness[i][j] + self.sturdiness[i + 1][j]) / 2;
                    let text = create_text(x, y, 2, &s.to_string(), "blue");
                    doc = doc.add(text);
                }
                if j != self.nx - 1 {
                    let x = (self.xs[j] + self.xs[j + 1]) / 2 + MARGIN;
                    let y = self.ys[i] + MARGIN;
                    let s = (self.sturdiness[i][j] + self.sturdiness[i][j + 1]) / 2;
                    let text = create_text(x, y, 2, &s.to_string(), "blue");
                    doc = doc.add(text);
                }
            }
        }
        svg::save("boring.svg", &doc).unwrap();
    }
}

/// ソルバー
struct Solver {
    n: usize, // 土地のサイズ、 n = 200
    w: usize, // 水源の数、1 <= w <= 4
    k: usize, // 家の数、 1 <= k <= 10
    c: usize, // 体力の消費, c in 1,2,4,8,16,32,64,128
    field: Field,
    boring_result: BoringResult,
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
        let field = Field::new(n, source_pos, house_pos);
        let boring_result = BoringResult::new(n, 20);
        Self {
            n,
            w,
            k,
            c,
            field,
            boring_result,
        }
    }

    pub fn solve(&mut self) {
        // プリム法で探索
        // マンハッタン距離
        // let dist = self.field.dist(|px, py| px.manhattan_dist(&py));
        // 試掘の結果を使う
        let dist = self.field.dist(|px, py| self.boring_result.dist(px, py));

        self.field.prim(dist);
        for i in 0..self.k {
            let start = self.field.house_pos[i];
            let goal = if self.field.conn[i] < self.w {
                self.field.source_pos[self.field.conn[i]]
            } else {
                self.field.house_pos[self.field.conn[i] - self.w]
            };

            let mut walk = Vec::new();
            walk.push(start);
            let start_near = self.boring_result.near_best_idx(start);
            let goal_near = self.boring_result.near_best_idx(goal);
            let mut shortest_path = self.boring_result.shortest_path(start_near, goal_near);
            walk.append(&mut shortest_path);
            walk.push(goal);

            for i in 0..walk.len() - 1 {
                self.walk(walk[i], walk[i + 1]);
            }
        }
    }

    /// startからgoalに向かう。
    pub fn walk(&mut self, start: Pos, goal: Pos) {
        // コメント
        println!(
            "# waok from ({} {}) to ({} {})",
            start.y, start.x, goal.y, goal.x
        );

        let mut pos = start;
        self.destruct(pos);
        // y方向に進んでから、x方向に進む
        while pos != goal {
            if pos.y < goal.y {
                pos.y += 1;
            } else if pos.y > goal.y {
                pos.y -= 1;
            } else if pos.x < goal.x {
                pos.x += 1;
            } else {
                pos.x -= 1;
            }
            self.destruct(pos);
        }
    }

    pub fn is_broken(&self, pos: Pos) -> bool {
        self.field.is_broken[pos.y as usize][pos.x as usize]
    }

    pub fn boring(&mut self) {
        // とりあえず50固定で5回まで掘ってみる
        // 11 * 11 * (C + 50) * 5 = 30250 + 605 * C のコストが最大かかる
        for i in 0..self.boring_result.ys.len() {
            for j in 0..self.boring_result.xs.len() {
                let mut n_try = 0;
                let pos = Pos {
                    y: self.boring_result.ys[i],
                    x: self.boring_result.xs[j],
                };
                while !self.is_broken(pos) && n_try < 5 {
                    n_try += 1;
                    self.try_destruct(pos, 50);
                }
                self.boring_result.sturdiness[i][j] =
                    self.field.power[pos.y as usize][pos.x as usize];
                if !self.is_broken(pos) {
                    // 壊れていない
                    self.boring_result.sturdiness[i][j] = 500;
                }
            }
        }
    }

    /// 掘るときの力
    pub fn destruct_power(&self, pos: Pos) -> usize {
        // 100固定
        100
    }

    /// 掘る
    pub fn destruct(&mut self, pos: Pos) {
        let power = self.destruct_power(pos);
        while !self.is_broken(pos) {
            self.try_destruct(pos, power);
        }
    }

    pub fn try_destruct(&mut self, pos: Pos, power: usize) {
        let result = self.field.query(&pos, power);
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

/// メイン関数
/// ../target/release/tester cargo run < tools/in/0000.txt > tools/out/0000.txt
fn main() {
    let stdin = std::io::stdin();
    let mut source = LineSource::new(BufReader::new(stdin));

    input! {
        from &mut source,
        n: usize,
        w: usize,
        k: usize,
        c: usize,
        ab: [(i64, i64); w],
        cd: [(i64, i64); k],
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
    solver.boring();
    solver.boring_result.calculate_dist();
    // solver.boring_result.save_svg();
    solver.solve();
    // solver.field.save_svg();
}
