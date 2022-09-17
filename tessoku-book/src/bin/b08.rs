use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n :usize,
        xy: [(Usize1, Usize1); n],
        q: usize,
        abcd: [(Usize1, Usize1, Usize1, Usize1); q],
    }

    let mut plane = vec![vec![0; 1500]; 1500];
    for (x, y) in xy {
        plane[x][y] += 1;
    }

    let mut cumsum = vec![vec![0; 1500 + 1]; 1500 + 1];

    // horizontal
    for i in 0..1500 {
        for j in 0..1500 {
            cumsum[i + 1][j + 1] = cumsum[i + 1][j] + plane[i][j];
        }
    }
    // vertical
    for j in 0..1500 {
        for i in 0..1500 {
            cumsum[i + 1][j + 1] += cumsum[i][j + 1];
        }
    }
    for (a, b, c, d) in abcd {
        let v = cumsum[a][b] + cumsum[c + 1][d + 1] - cumsum[a][d + 1] - cumsum[c + 1][b];
        println!("{}", v);
    }
}
