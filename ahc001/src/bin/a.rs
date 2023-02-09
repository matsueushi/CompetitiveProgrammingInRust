use std::collections::BTreeSet;

use proconio::input;
use rand::{Rng, SeedableRng};

/// 解を捜索する
fn find_arrangement(input_data: &Input) -> Vec<(usize, usize, usize, usize)> {
    let mut agmt = Vec::new();
    for Request { x, y, size: _ } in &input_data.sps {
        agmt.push((*x, *y, *x + 1, *y + 1));
    }
    agmt
}

/// 入力関連

/// 企業が要求しているスペース
#[derive(Clone, Debug)]
pub struct Request {
    x: usize,
    y: usize,
    size: usize,
}

/// 入力を保持するクラス
pub struct Input {
    sps: Vec<Request>,
}

/// 入力を読み込む
fn read_input() -> Input {
    input! {
        n: usize,
        xyr: [(usize, usize, usize); n],
    }
    let mut sps = Vec::new();
    for (x, y, size) in xyr {
        sps.push(Request { x, y, size });
    }
    Input { sps }
}

/// 入力を生成する
#[warn(dead_code)]
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
    let mut q = rand::seq::index::sample(&mut rng, 10000 * 10000 - 1, n - 1)
        .into_iter()
        .map(|a| a + 1)
        .collect::<Vec<_>>();
    q.push(0);
    q.push(10000 * 10000);
    q.sort();

    let mut size = Vec::new();
    for i in 0..n {
        size.push(q[i + 1] - q[i]);
    }

    let mut sps = Vec::new();
    for i in 0..n {
        sps.push(Request {
            x: ps[i].0,
            y: ps[i].1,
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
    // let input_data = read_input();

    // 自動生成
    let input_data = generate_input(0);

    let solution = find_arrangement(&input_data);
    for (a, b, c, d) in solution {
        println!("{} {} {} {}", a, b, c, d);
    }
}
