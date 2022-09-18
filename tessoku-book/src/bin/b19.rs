use proconio::{fastout, input};

const VMAX: usize = 100_000;

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }
    let mut dp = vec![vec![std::usize::MAX / 2; VMAX + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let (wi, vi) = wv[i];
        for j in 0..=VMAX {
            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
            if vi + j <= VMAX {
                dp[i + 1][vi + j] = dp[i + 1][vi + j].min(dp[i][j] + wi);
            }
        }
    }

    let mut res = 0;
    for j in 0..=VMAX {
        if dp[n][j] <= w {
            res = j;
        }
    }
    println!("{}", res);
}
