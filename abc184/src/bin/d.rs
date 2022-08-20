use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let mut prob = vec![vec![vec![0.0; 101]; 101]; 101];
    for i in (0..100).rev() {
        for j in (0..100).rev() {
            for k in (0..100).rev() {
                if i == 0 && j == 0 && k == 0 {
                    continue;
                }
                let d = (i + j + k) as f64;
                prob[i][j][k] = 1.0
                    + (i as f64) / d * prob[i + 1][j][k]
                    + (j as f64) / d * prob[i][j + 1][k]
                    + (k as f64) / d * prob[i][j][k + 1];
            }
        }
    }
    println!("{}", prob[a][b][c]);
}
