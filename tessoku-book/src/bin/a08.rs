use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[usize; w]; h],
        q: usize,
        abcd: [(Usize1, Usize1, Usize1, Usize1); q],
    }
    let mut cumsum = vec![vec![0; w + 1]; h + 1];

    // horizontal
    for i in 0..h {
        for j in 0..w {
            cumsum[i + 1][j + 1] = cumsum[i + 1][j] + x[i][j];
        }
    }
    // vertical
    for j in 0..w {
        for i in 0..h {
            cumsum[i + 1][j + 1] += cumsum[i][j + 1];
        }
    }
    for (a, b, c, d) in abcd {
        let v = cumsum[a][b] + cumsum[c + 1][d + 1] - cumsum[a][d + 1] - cumsum[c + 1][b];
        println!("{}", v);
    }
}
