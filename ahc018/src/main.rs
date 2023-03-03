use petgraph::unionfind::UnionFind;
use proconio::{input, source::line::LineSource};
use std::io::BufReader;
use std::process;
use text_io::read;

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
    power: Vec<Vec<usize>>,    // 掘削するのに消費したパワー
    n_trial: Vec<Vec<usize>>,  // 掘削した回数
    is_broken: Vec<Vec<bool>>, // すでにフィールドが壊れているか
}

impl Field {
    pub fn new(n: usize, c: usize) -> Self {
        Self {
            power: vec![vec![0; n]; n],
            n_trial: vec![vec![0; n]; n],
            is_broken: vec![vec![false; n]; n],
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
}

/// ソルバー
#[allow(unused)]
struct Solver {
    n: usize, // 土地のサイズ、 n = 200
    w: usize, // 水源の数、1 <= w <= 4
    k: usize, // 家の数、 1 <= k <= 10
    c: usize, // 体力の消費, c in 1,2,4,8,16,32,64,128
    source_pos: Vec<Pos>,
    house_pos: Vec<Pos>,
    field: Field,
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
        let field = Field::new(n, c);
        Self {
            n,
            w,
            k,
            c,
            source_pos,
            house_pos,
            field,
        }
    }

    pub fn solve(&mut self) {
        for house in self.house_pos.clone() {
            self.walk(house, self.source_pos[0]);
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

    // 試掘を行う。
    // pub fn boring(&self) {
    //     visualize_boring()
    // }
}

/// ボーリングを行うボーリングミル
// struct BoringMill {
//     cost: Vec<Vec<usize>>,
// }

// impl BoringMill {}

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
    // solver.boring();
    solver.solve();
}

///
/// 以下、考察用
///
#[cfg(feature = "local")]
use svg::node::element::{path::Data, Path};

#[cfg(not(feature = "local"))]
#[allow(dead_code)]
fn visualize_boring() {}

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
#[allow(dead_code)]
fn visualize_boring() {
    let margin = 20;
    let w = 200;
    let d = w + 2 * margin;
    let mut doc = svg::Document::new().set("viewBox", (0, 0, d, d));
    let back = Path::new().set("fill", "white").set(
        "d",
        create_rect_data(margin, margin, margin + w, margin + w),
    );
    doc = doc.add(back);
    let text = svg::node::element::Text::new()
        .set("x", margin)
        .set("y", margin)
        .set("font-size", 10)
        .add(svg::node::Text::new("aaa"));
    doc = doc.add(text);
    svg::save("boring.svg", &doc).unwrap();
}
