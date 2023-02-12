use num_integer::Roots;
use rand::{thread_rng, Rng, SeedableRng};
use std::collections::BTreeSet;

use proconio::input;
use svg::node::element::{path::Data, Path};

const W: usize = 10000;
const MAX_ITER: usize = 10000;
const VERBOSE: usize = 10; // debug

/// 点
#[derive(Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    /// 動かす
    fn move_by(&mut self, dx: i64, dy: i64) {
        self.x += dx;
        self.y += dy;
    }
}

/// 長方形
#[derive(Clone, Debug)]
struct Rect {
    p0: Point,
    p1: Point,
}

impl Rect {
    fn area(&self) -> i64 {
        (self.p1.x - self.p0.x) * (self.p1.y - self.p0.y)
    }

    /// 交差しているか判定
    fn intersect(&self, other: &Rect) -> bool {
        self.p0.x.max(other.p0.x) < self.p1.x.min(other.p1.x)
            && self.p0.y.max(other.p0.y) < self.p1.y.min(other.p1.y)
    }

    /// 領域に収まっているか
    fn out_of_range(&self) -> bool {
        self.p0.x < 0 || self.p1.x > W as i64 || self.p0.y < 0 || self.p1.y > W as i64
    }

    /// 座標の順序が正しいか
    fn is_valid(&self) -> bool {
        self.p0.x < self.p1.x && self.p0.y < self.p1.y
    }

    /// 点を含むか
    pub fn contains(&self, p: &Point) -> bool {
        self.p0.x <= p.x && p.x <= self.p1.x && self.p0.y <= p.y && p.y <= self.p1.y
    }

    /// 動かす
    pub fn move_by(&mut self, dx0: i64, dy0: i64, dx1: i64, dy1: i64) {
        self.p0.move_by(dx0, dy0);
        self.p1.move_by(dx1, dy1);
    }

    /// 変換
    pub fn transform(&mut self, i: usize, s: i64) {
        match i {
            0 => self.move_by(s, 0, 0, 0),
            1 => self.move_by(0, s, 0, 0),
            2 => self.move_by(0, 0, s, 0),
            3 => self.move_by(0, 0, 0, s),
            4 => self.move_by(-s, 0, 0, 0),
            5 => self.move_by(0, -s, 0, 0),
            6 => self.move_by(0, 0, -s, 0),
            7 => self.move_by(0, 0, 0, -s),
            _ => unreachable!(),
        }
    }

    /// 中心
    pub fn center(&self) -> Point {
        Point {
            x: (self.p0.x + self.p1.x) / 2,
            y: (self.p0.y + self.p1.y) / 2,
        }
    }
}

/// 解のアレンジメント
type Arrangement = Vec<Rect>;

/// 要求に対して与えた面積の割合
fn area_fill_ratio(r: i64, s: i64) -> f64 {
    let r = r as f64;
    let s = s as f64;
    r.min(s) / r.max(s)
}

/// スコアを計算する
fn eval_score(input_data: &Input, rects: &Arrangement) -> i64 {
    let n = input_data.len();
    let mut score = 0.0;
    for i in 0..n {
        if rects[i].out_of_range() {
            // out of range
            // eprintln!("out of range: {}", i);
            return 0;
        }
        if !rects[i].is_valid() {
            // negative area
            // eprintln!("negative area: {}", i);
            return 0;
        }
        if !rects[i].contains(&input_data[i].p) {
            // 点を含まない場合はスコアには関与しない
            // eprintln!("point is not included: {}", i);
            continue;
        }
        for j in 0..i {
            if rects[i].intersect(&rects[j]) {
                // オーバーラップしているものがある
                // eprintln!("overlap: {} {}", i, j);
                return 0;
            }
        }
        let ti = area_fill_ratio(rects[i].area(), input_data[i].area);
        score += 1.0 - (1.0 - ti) * (1.0 - ti);
    }
    (1e9 * score / n as f64).round() as i64
}

/// 解を捜索する
fn find_arrangement(input_data: &Input) -> Arrangement {
    let mut rects = Vec::new();
    // スコアの履歴
    let mut score_hist = Vec::new();

    // 最初の解を探索する
    for Request {
        p: Point { x, y },
        area: _,
    } in input_data
    {
        rects.push(Rect {
            p0: Point {
                x: *x as i64,
                y: *y as i64,
            },
            p1: Point {
                x: (*x + 1) as i64,
                y: (*y + 1) as i64,
            },
        });
    }
    let mut score = eval_score(input_data, &rects);
    let mut rng = thread_rng();
    let n = input_data.len();

    for round in 0..MAX_ITER {
        // eprintln!("Score: {}", score);
        let pos = rng.gen_range(0, n);
        let state = rng.gen_range(0, 8);
        // 面積の差分
        let area_diff = (input_data[pos].area - rects[pos].area())
            .abs()
            .sqrt()
            .max(2);
        let enl = rng.gen_range(1, area_diff);
        rects[pos].transform(state, enl);
        let new_score = eval_score(input_data, &rects);

        if new_score >= score {
            score = new_score;
        } else {
            rects[pos].transform(state, -enl);
        }

        // スコア
        score_hist.push(score);

        // 学習の履歴
        if round % VERBOSE == 0 {
            visualize(&input_data, &rects, round);
        }
    }

    rects
}

/// 入力関連

/// 企業が要求しているスペース
#[derive(Clone, Debug)]
pub struct Request {
    p: Point,
    area: i64,
}

/// 入力を保持するクラス
type Input = Vec<Request>;

/// 入力を読み込む
#[allow(dead_code)]
fn read_input() -> Input {
    input! {
        n: usize,
        xyr: [(i64, i64, i64); n],
    }
    let mut sps = Vec::new();
    for (x, y, area) in xyr {
        sps.push(Request {
            p: Point { x, y },
            area,
        });
    }
    sps
}

/// 入力を生成する
#[allow(dead_code)]
fn generate_input(state: u64) -> Input {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(state);
    let n = (50.0 * 4.0f64.powf(rng.gen::<f64>())).round() as usize;
    let mut ps = Vec::new();
    let mut used = BTreeSet::new();
    for _ in 0..n {
        loop {
            let x = rng.gen_range(0, 10000);
            let y = rng.gen_range(0, 10000);
            if used.insert((x, y)) {
                ps.push((x, y));
                break;
            }
        }
    }
    let mut q = rand::seq::index::sample(&mut rng, W * W - 1, n - 1)
        .into_iter()
        .map(|a| a + 1)
        .collect::<Vec<_>>();
    q.push(0);
    q.push(W * W);
    q.sort();

    let mut area = Vec::new();
    for i in 0..n {
        area.push((q[i + 1] - q[i]) as i64);
    }

    let mut sps = Vec::new();
    for i in 0..n {
        sps.push(Request {
            p: Point {
                x: ps[i].0,
                y: ps[i].1,
            },
            area: area[i],
        });
    }
    sps
}

/// 可視化関連
#[allow(dead_code)]
fn rect(r: &Rect) -> Data {
    Data::new()
        .move_to((r.p0.x, r.p0.y))
        .line_by((r.p1.x - r.p0.x, 0))
        .line_by((0, r.p1.y - r.p0.y))
        .line_by((r.p0.x - r.p1.x, 0))
        .close()
}

// 0 <= val <= 1
#[allow(dead_code)]
fn fill_color(val: f64) -> String {
    let tmp = ((-(2.0 * std::f64::consts::PI * val).cos() / 2.0 + 0.5) * 255.0) as i32;
    if val >= 0.5 {
        format!("#{:02x}{:02x}{:02x}", 255, 0, tmp)
    } else {
        format!("#{:02x}{:02x}{:02x}", tmp, 0, 255)
    }
}

#[allow(dead_code)]
fn visualize(input_data: &Input, rects: &Vec<Rect>, round: usize) {
    let mut doc = svg::Document::new().set("viewBox", (0, 0, W, W));
    doc = doc.add(Path::new().set("fill", "white").set(
        "d",
        rect(&Rect {
            p0: Point { x: 0, y: 0 },
            p1: Point {
                x: W as i64,
                y: W as i64,
            },
        }),
    ));
    let fontsize = 200.0 / (input_data.len() as f64 / 50.0).sqrt();
    for i in 0..input_data.len() {
        let px = input_data[i].p.x;
        let py = input_data[i].p.y;

        // rect
        let val = if rects[i].area() > input_data[i].area {
            1.0 - input_data[i].area as f64 / rects[i].area() as f64 / 2.0
        } else {
            rects[i].area() as f64 / input_data[i].area as f64 / 2.0
        };
        let path = Path::new()
            .set("fill", fill_color(val))
            .set("stroke", "black")
            .set("stroke-width", 5.0)
            .set("d", rect(&rects[i]));
        doc = doc.add(path);
        // id
        let text = svg::node::element::Text::new()
            .set("x", px as f64)
            .set("y", py as f64 + fontsize * 0.35)
            .set("font-size", fontsize)
            .add(svg::node::Text::new(format!("{}", i)));
        doc = doc.add(text);
        // origin
        let data = rect(&Rect {
            p0: Point {
                x: px - 30,
                y: py - 30,
            },
            p1: Point {
                x: px + 30,
                y: py + 30,
            },
        });
        let path = Path::new().set("fill", "green").set("d", data);
        doc = doc.add(path);
        // line
        let center = rects[i].center();
        let data = Data::new()
            .move_to((px, py))
            .line_by((center.x - px, center.y - py));
        let path = Path::new()
            .set("stroke", "black")
            .set("stroke-width", 5.0)
            .set("d", data);
        doc = doc.add(path);
        // score
        let ti = area_fill_ratio(rects[i].area(), input_data[i].area);
        let text = svg::node::element::Text::new()
            .set("x", px as f64 + fontsize * 0.7)
            .set("y", py as f64 + fontsize * 0.7)
            .set("font-size", fontsize * 0.5)
            .add(svg::node::Text::new(format!("{:.3}", ti)));
        doc = doc.add(text);
    }
    let save_path = format!("history/{:05}.svg", round);
    svg::save(save_path, &doc).unwrap();
}

/// エントリーポイント
/// 実行するときは、
/// cat tools/in/0000.txt | cargo run --bin ahc001-a > tools/out/0000.txt
fn main() {
    // 標準入力
    let input_data = read_input();

    // 自動生成
    // let input_data = generate_input(0);

    let solution = find_arrangement(&input_data);
    for Rect {
        p0: Point { x: x0, y: y0 },
        p1: Point { x: x1, y: y1 },
    } in solution
    {
        println!("{} {} {} {}", x0, y0, x1, y1);
    }
}
