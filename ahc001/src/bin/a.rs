use rand::{thread_rng, Rng, SeedableRng};
use std::collections::BTreeSet;

use proconio::input;
use svg::node::element::{path::Data, Path};

const W: usize = 10000;
const MAX_ITER: usize = 2500;
const VERBOSE: usize = 10;

/// 点
#[derive(Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
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

// fn temperature(round: usize) -> f64 {
//     30.0 - 28.0 * round as f64 / MAX_ITER as f64
// }

/// スコアを計算する
fn eval_score(input_data: &Input, agmt: &Arrangement) -> i64 {
    let n = input_data.len();
    let mut score = 0.0;
    for i in 0..n {
        if agmt[i].out_of_range() {
            // out of range
            // eprintln!("out of range: {}", i);
            return 0;
        }
        if !agmt[i].is_valid() {
            // negative area
            // eprintln!("negative area: {}", i);
            return 0;
        }
        if !agmt[i].contains(&input_data[i].p) {
            // 点を含まない場合はスコアには関与しない
            // eprintln!("point is not included: {}", i);
            continue;
        }
        for j in 0..i {
            if agmt[i].intersect(&agmt[j]) {
                // オーバーラップしているものがある
                // eprintln!("overlap: {} {}", i, j);
                return 0;
            }
        }
        let ri = agmt[i].area() as f64;
        let si = input_data[i].area as f64;
        let ti = ri.min(si) / ri.max(si);
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

    let dx0 = [1, 0, -1, 0, 0, 0, 0, 0];
    let dy0 = [0, 1, 0, -1, 0, 0, 0, 0];
    let dx1 = [0, 0, 0, 0, 1, 0, -1, 0];
    let dy1 = [0, 0, 0, 0, 0, 1, 0, -1];

    for round in 0..MAX_ITER {
        // eprintln!("Score: {}", score);
        let pos = rng.gen_range(0, n);
        let state = rng.gen_range(0, 8);
        let enl = rng.gen_range(1, 100);
        rects[pos].p0.x += dx0[state] * enl;
        rects[pos].p0.y += dy0[state] * enl;
        rects[pos].p1.x += dx1[state] * enl;
        rects[pos].p1.y += dy1[state] * enl;
        let new_score = eval_score(input_data, &rects);

        if new_score >= score {
            score = new_score;
        } else {
            rects[pos].p0.x -= dx0[state] * enl;
            rects[pos].p0.y -= dy0[state] * enl;
            rects[pos].p1.x -= dx1[state] * enl;
            rects[pos].p1.y -= dy1[state] * enl;
        }

        // 学習の履歴
        if VERBOSE > 0 && round % VERBOSE == 0 {
            score_hist.push(score);
            visualize(&input_data, &rects);
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
fn rect(r: &Rect) -> Data {
    Data::new()
        .move_to((r.p0.x, r.p0.y))
        .line_by((r.p1.x - r.p0.x, 0))
        .line_by((0, r.p1.y - r.p0.y))
        .line_by((r.p0.x - r.p1.x, 0))
        .close()
}

fn visualize(input_data: &Input, rects: &Vec<Rect>) {
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
        let center = rects[i].center();
        doc = doc.add(
            svg::node::element::Text::new()
                .set("x", center.x as f64)
                .set("y", center.y as f64 + fontsize * 0.35)
                .set("font-size", fontsize)
                .set("text-anchor", "middle")
                .add(svg::node::Text::new(format!("{}", i))),
        )
    }
    svg::save("a.svg", &doc).unwrap();
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
