use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        abcd: [(Usize1, Usize1, Usize1, Usize1); n],
    }

    let h = 1500;
    let w = 1500;

    let mut plane = vec![vec![0; w + 1]; h + 1];
    for (a, b, c, d) in abcd {
        plane[a][b] += 1;
        plane[a][d] -= 1;
        plane[c][b] -= 1;
        plane[c][d] += 1;
    }

    let mut cumsum = vec![vec![0; w + 1]; h + 1];

    // horizontal
    for i in 0..h {
        for j in 0..w {
            cumsum[i + 1][j + 1] = cumsum[i + 1][j] + plane[i][j];
        }
    }
    // vertical
    for j in 0..w {
        for i in 0..h {
            cumsum[i + 1][j + 1] += cumsum[i][j + 1];
        }
    }

    let mut res = 0;
    for i in 1..=h {
        for j in 1..=w {
            if cumsum[i][j] > 0 {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
