use proconio::input;
use rand::{Rng, SeedableRng};
use std::collections::BTreeSet;

const W: usize = 10000;
const MAX_ITER: usize = 1_000_000;

/// 点
#[derive(Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn initial_rect(&self) -> Rect {
        Rect {
            p0: Self {
                x: self.x,
                y: self.y,
            },
            p1: Self {
                x: self.x + 1,
                y: self.y + 1,
            },
        }
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
        // 点 (p.x + 0.5, p.y + 0.5) を含まなくてはならない
        self.p0.x <= p.x && p.x < self.p1.x && self.p0.y <= p.y && p.y < self.p1.y
    }

    /// 動かす
    pub fn extend(&self, dx0: i64, dy0: i64, dx1: i64, dy1: i64) -> Self {
        Self {
            p0: Point {
                x: (self.p0.x + dx0).max(0),
                y: (self.p0.y + dy0).min(W as i64),
            },
            p1: Point {
                x: (self.p1.x + dx1).max(0),
                y: (self.p1.y + dy1).min(W as i64),
            },
        }
    }

    /// 変換
    pub fn transform(&self, i: usize, s: i64) -> Self {
        match i {
            0 => self.extend(s, 0, 0, 0),
            1 => self.extend(0, s, 0, 0),
            2 => self.extend(0, 0, s, 0),
            3 => self.extend(0, 0, 0, s),
            4 => self.extend(s, 0, s, 0),
            5 => self.extend(0, s, 0, s),
            6 => self.extend(s, 0, -s, 0),
            7 => self.extend(0, s, 0, -s),
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

struct ScoreCalculator {
    rect_scores: Vec<f64>,
    raw_score: f64,
}

impl ScoreCalculator {
    fn new(input_data: &Input) -> Self {
        let mut rect_scores = Vec::new();
        for r in input_data {
            let t = area_fill_ratio(1, r.area);
            let s = 1.0 - (1.0 - t) * (1.0 - t);
            rect_scores.push(s);
        }
        let raw_score = rect_scores.iter().sum();
        Self {
            rect_scores,
            raw_score,
        }
    }

    fn score(&self) -> i64 {
        (1e9 * self.raw_score / self.rect_scores.len() as f64).round() as i64
    }

    fn update(
        &mut self,
        input_data: &Input,
        rects: &Arrangement,
        i: usize,
        rect: &Rect,
        threshold: f64,
    ) -> Result<f64, f64> {
        if rect.out_of_range() {
            // out of range
            // eprintln!("out of range: {}", i);
            return Err(self.raw_score);
        }
        if !rect.is_valid() {
            // negative area
            // eprintln!("negative area: {}", i);
            return Err(self.raw_score);
        }
        if !rect.contains(&input_data[i].p) {
            // 点を含まない場合はスコアには関与しない
            // eprintln!("point is not included: {}", i);
            // スコアに関与しないが、ペナルティとして0にしてしまう
            return Err(self.raw_score);
        }
        for j in 0..rects.len() {
            if i == j {
                continue;
            }
            if rect.intersect(&rects[j]) {
                // オーバーラップしているものがある
                // eprintln!("overlap: {} {}", i, j);
                return Err(self.raw_score);
            }
        }

        let t = area_fill_ratio(rect.area(), input_data[i].area);
        let s = 1.0 - (1.0 - t) * (1.0 - t);
        if self.rect_scores[i] - s > threshold {
            return Err(self.raw_score);
        }

        self.rect_scores[i] = s;
        self.raw_score = self.rect_scores.iter().sum();
        Ok(self.raw_score)
    }
}

/// 解を捜索する
fn find_arrangement(input_data: &Input) -> Arrangement {
    let mut rects = Vec::new();
    // スコアの履歴
    let mut score_hist = Vec::new();

    // 最初の解を探索する
    for rect in input_data {
        rects.push(rect.p.initial_rect());
    }
    let mut score_calculator = ScoreCalculator::new(input_data);
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(42);
    let n = input_data.len();

    for round in 0..MAX_ITER {
        let pos = rng.gen_range(0, n);

        let state = rng.gen_range(0, 8);
        let enl = rng.gen_range(-100, 100);

        let rect = rects[pos].transform(state, enl);
        let ret = score_calculator.update(input_data, &rects, pos, &rect, 0.001);
        if ret.is_ok() {
            rects[pos] = rect;
        }

        // スコア
        score_hist.push(score_calculator.score());
        // 可視化する
        visualize(&input_data, &rects, round, score_calculator.score());
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

/// エントリーポイント
/// 実行するときは、
/// cat tools/in/0000.txt | cargo run --release --bin ahc001-a > tools/out/0000.txt
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

#[cfg(not(feature = "local"))]
fn visualize(input_data: &Input, rects: &Vec<Rect>, round: usize, score: i64) {}

///
/// 下はデバッグ用
///

/// 可視化関連
#[cfg(feature = "local")]
use svg::node::element::{path::Data, Path};
#[cfg(feature = "local")]
// const VERBOSE: Option<usize> = None;
const VERBOSE: Option<usize> = Some(1000); // debug

#[cfg(feature = "local")]
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
#[cfg(feature = "local")]
#[allow(dead_code)]
fn fill_color(val: f64) -> String {
    let tmp = ((-(2.0 * std::f64::consts::PI * val).cos() / 2.0 + 0.5) * 255.0) as i32;
    if val >= 0.5 {
        format!("#{:02x}{:02x}{:02x}", 255, 0, tmp)
    } else {
        format!("#{:02x}{:02x}{:02x}", tmp, 0, 255)
    }
}

#[cfg(feature = "local")]
#[allow(dead_code)]
fn visualize(input_data: &Input, rects: &Vec<Rect>, round: usize, score: i64) {
    match VERBOSE {
        None => return,
        Some(n) => {
            if round % n != 0 {
                return;
            }
        }
    }

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
        // failed
        let text = svg::node::element::Text::new()
            .set("stroke", "blue")
            .set("x", 200)
            .set("y", 200)
            .set("font-size", fontsize)
            .add(svg::node::Text::new(format!("{}, {}", round, score)));
        doc = doc.add(text);
        // id, score for ad
        let ti = area_fill_ratio(rects[i].area(), input_data[i].area);
        let text = svg::node::element::Text::new()
            .set("x", px as f64)
            .set("y", py as f64 + fontsize * 0.35)
            .set("font-size", fontsize)
            .add(svg::node::Text::new(format!("{}: {:.3}", i, ti)));
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
    }
    let save_path = format!("history/{:05}.svg", round);
    svg::save(save_path, &doc).unwrap();
}
