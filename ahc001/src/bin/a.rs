use std::collections::BTreeSet;

use proconio::input;
use rand::{thread_rng, Rng, SeedableRng};

const W: usize = 10000;
const MAX_ITER: usize = 1000;

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
}

/// 解のアレンジメント
type Arrangement = Vec<Rect>;

/// 解を捜索する
fn find_arrangement(input_data: &Input) -> Arrangement {
    let mut rects = Vec::new();

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

    let dx0 = [1, 0, -1, 0, 0, 0, 0, 0, 1, 0, -1, 0, -1, 0, 1, 0];
    let dy0 = [0, 1, 0, -1, 0, 0, 0, 0, 0, 1, 0, -1, 0, -1, 0, -1];
    let dx1 = [0, 0, 0, 0, 1, 0, -1, 0, -1, 0, 1, 0, 1, 0, -1, 0];
    let dy1 = [0, 0, 0, 0, 0, 1, 0, -1, 0, -1, 0, 1, 0, 1, 0, -1];

    for _ in 0..MAX_ITER {
        // eprintln!("Score: {}", score);
        let pos = rng.gen_range(0, n);
        let state = rng.gen_range(0, 16);
        let enl = rng.gen_range(1, 1000);
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
    }

    rects
}

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
