use proconio::input;

/// 入力を読み込む
fn read_input() -> Vec<(usize, usize, usize)> {
    input! {
        n: usize,
        xyr: [(usize, usize, usize); n],
    }
    xyr
}

/// 解を捜索する
fn find_arrangement(input_data: &Vec<(usize, usize, usize)>) -> Vec<(usize, usize, usize, usize)> {
    let mut agmt = Vec::new();
    for (x, y, r) in input_data {
        agmt.push((*x, *y, *x + 1, *y + 1));
    }
    agmt
}

/// 入力データ

/// エントリーポイント
/// 実行するときは、
/// cat tools/in/0000.txt | cargo run --bin ahc001-a > tools/out/0000.txt
fn main() {
    let input_data = read_input();
    let solution = find_arrangement(&input_data);
    for (a, b, c, d) in solution {
        println!("{} {} {} {}", a, b, c, d);
    }
}
