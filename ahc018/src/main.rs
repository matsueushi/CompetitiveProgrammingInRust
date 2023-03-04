use proconio::{input, source::line::LineSource};
use std::io::BufReader;
use std::process;
#[cfg(feature = "local")]
use svg::node::element::{path::Data, Path, Text, SVG};
use text_io::read;

const INF: usize = std::usize::MAX / 2;

/// 場所を示す構造体。
#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct Pos {
    y: i64,
    x: i64,
}

impl Pos {
    /// マンハッタン距離
    pub fn manhattan_dist(&self, other: &Self) -> i64 {
        (self.y - other.y).abs() + (self.x - other.y).abs()
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

#[allow(unused)]
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

    pub fn prim(&mut self) {
        // まず、距離を調べる
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
            for j in 0..i {
                let d = self.house_pos[self.w + i].manhattan_dist(&self.house_pos[self.w + j]);
                dist[self.w + i][self.w + j] = d as usize;
                dist[self.w + j][self.w + i] = d as usize;
            }
            // 水源と家との距離
            for l in 0..self.w {
                let d = self.house_pos[self.w + i].manhattan_dist(&self.source_pos[l]);
                dist[self.w + i][l] = d as usize;
                dist[l][self.w + i] = d as usize;
            }
        }

        // Prim法
        let mut min_cost = vec![INF; self.k];

        loop {
            // X に属さない頂点からコストが最小になる点を探す
            let mut v = None;
            for u in 0..self.k {
                if self.conn[u] < INF {
                    continue;
                }
                match v {
                    Some(i) => {
                        if min_cost[u] < min_cost[i] {
                            v = Some(u)
                        }
                    }
                    None => v = Some(u),
                }
            }

            // コストが最小になる点を追加し、距離を更新する
            match v {
                Some(i) => {
                    for u in 0..self.w + self.k {
                        if dist[u][i] < min_cost[i] {
                            min_cost[i] = dist[u][i];
                            self.conn[i] = u;
                        }
                    }
                }
                None => break,
            }
        }
    }

    #[cfg(not(feature = "local"))]
    pub fn svg(&self) -> SVG {}

    #[cfg(feature = "local")]
    pub fn svg(&self) -> SVG {
        const MARGIN: usize = 20;
        use svg::node::Text;

        fn create_rect_data(y0: usize, x0: usize, y1: usize, x1: usize) -> Data {
            Data::new()
                .move_to((x0, y0))
                .line_by((x1 as i64 - x0 as i64, 0))
                .line_by((0, y1 as i64 - y0 as i64))
                .line_by((x0 as i64 - x1 as i64, 0))
                .close()
        }

        fn create_line(x1: i64, y1: i64, x2: i64, y2: i64) -> svg::node::element::Line {
            svg::node::element::Line::new()
                .set("x1", x1 as usize)
                .set("y1", y1 as usize)
                .set("x2", x2 as usize)
                .set("y2", y2 as usize)
                .set("stroke", "black")
                .set("stroke-width", 1)
        }

        fn create_circ(x: i64, y: i64, r: usize, stroke: &str) -> svg::node::element::Circle {
            svg::node::element::Circle::new()
                .set("cx", x as usize)
                .set("cy", y as usize)
                .set("r", r)
                .set("stroke", stroke)
                .set("stroke-width", 1)
                .set("fill", "transparent")
        }

        fn create_text(x: i64, y: i64, font_size: usize, text: &str) -> svg::node::element::Text {
            svg::node::element::Text::new()
                .set("x", x as usize + MARGIN - font_size / 2)
                .set("y", y as usize + MARGIN + font_size / 2)
                .set("font-size", font_size)
                .set("font_family", "monospace")
                .add(svg::node::Text::new(text))
        }

        let w = 200;
        let d = w + 2 * MARGIN;
        let mut doc = svg::Document::new().set("viewBox", (0, 0, d, d));
        let back = Path::new()
            .set("fill", "white")
            .set("d", create_rect_data(0, 0, w, w));
        doc = doc.add(back);
        for pos in &self.source_pos {
            let circ = create_circ(pos.x, pos.y, 5, "blue");
            doc = doc.add(circ);
        }
        for pos in &self.house_pos {
            let circ = create_circ(pos.x, pos.y, 5, "green");
            doc = doc.add(circ);
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
            );
            doc = doc.add(text);
        }
        doc
    }
}

/// ソルバー
#[allow(unused)]
struct Solver {
    n: usize, // 土地のサイズ、 n = 200
    w: usize, // 水源の数、1 <= w <= 4
    k: usize, // 家の数、 1 <= k <= 10
    c: usize, // 体力の消費, c in 1,2,4,8,16,32,64,128
    field: Field,
}

#[allow(unused)]
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
        Self { n, w, k, c, field }
    }

    pub fn solve(&mut self) {
        for house in self.field.house_pos.clone() {
            self.walk(house, self.field.source_pos[0]);
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

    /// 掘るときの力
    pub fn destruct_power(&self, pos: Pos) -> usize {
        // 100固定
        100
    }

    /// 掘る
    pub fn destruct(&mut self, pos: Pos) {
        let power = self.destruct_power(pos);
        while !self.field.is_broken[pos.y as usize][pos.x as usize] {
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

    pub fn save_svg(&self) {
        let doc = self.field.svg();
        svg::save("field.svg", &doc).unwrap();
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
    solver.save_svg();
    solver.solve();
}
