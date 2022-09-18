use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, i64); n],
    }
    let mut dp = vec![vec![std::i64::MIN; w + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let (wi, vi) = wv[i];
        for j in 0..=w {
            dp[i + 1][j] = dp[i][j];
            if j >= wi {
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j - wi] + vi);
            }
        }
    }

    let mut res = 0;
    for j in 0..=w {
        res = res.max(dp[n][j]);
    }
    println!("{}", res);
}
