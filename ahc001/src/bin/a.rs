use std::collections::BTreeSet;

use proconio::input;
use rand::{Rng, SeedableRng};

const W: usize = 10000;

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
        self.p0.x >= self.p1.x || self.p0.y >= self.p1.y
    }

    /// 点を含むか
    pub fn contains(&self, x: i64, y: i64) -> bool {
        self.p0.x <= x && x <= self.p1.x && self.p0.y <= y && y <= self.p1.y
    }
}

/// 解のアレンジメント
struct Arrangement {
    pub rects: Vec<Rect>,
}

/// 解を捜索する
fn find_arrangement(input_data: &Input) -> Arrangement {
    let mut rects = Vec::new();
    for Request {
        p: Point { x, y },
        size: _,
    } in &input_data.sps
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
    Arrangement { rects }
}

/// スコアを計算する
fn eval_score(input_data: &Input, agmt: &Arrangement) -> usize {
    let n = input_data.sps.len();
    let mut score = 0.0;
    for i in 0..n {
        if agmt.rects[i].out_of_range() {
            // out of range
            return 0;
        }
        if !agmt.rects[i].is_valid() {
            // negative area
            return 0;
        }
    }
    0
}

/// 入力関連

/// 企業が要求しているスペース
#[derive(Clone, Debug)]
pub struct Request {
    p: Point,
    size: usize,
}

/// 入力を保持するクラス
pub struct Input {
    sps: Vec<Request>,
}

/// 入力を読み込む
#[allow(dead_code)]
fn read_input() -> Input {
    input! {
        n: usize,
        xyr: [(i64, i64, usize); n],
    }
    let mut sps = Vec::new();
    for (x, y, size) in xyr {
        sps.push(Request {
            p: Point { x, y },
            size,
        });
    }
    Input { sps }
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

    let mut size = Vec::new();
    for i in 0..n {
        size.push(q[i + 1] - q[i]);
    }

    let mut sps = Vec::new();
    for i in 0..n {
        sps.push(Request {
            p: Point {
                x: ps[i].0,
                y: ps[i].1,
            },
            size: size[i],
        });
    }
    Input { sps }
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
    } in solution.rects
    {
        println!("{} {} {} {}", x0, y0, x1, y1);
    }
}

/// テスト
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect() {
        let rect0 = Rect {
            p0: Point { x: 0, y: 0 },
            p1: Point { x: 3, y: 3 },
        };
        let rect1 = Rect {
            p0: Point { x: 1, y: 1 },
            p1: Point { x: 4, y: 4 },
        };
        let rect2 = Rect {
            p0: Point { x: 0, y: 0 },
            p1: Point { x: 1, y: 4 },
        };

        assert_eq!(9, rect0.area());
        assert_eq!(9, rect1.area());
        assert_eq!(4, rect2.area());

        assert!(rect0.intersect(&rect1));
        assert!(!rect1.intersect(&rect2));
    }
}
