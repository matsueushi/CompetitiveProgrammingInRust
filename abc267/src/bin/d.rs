use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    }

    let mut dp = vec![vec![std::i64::MIN; n + 1]; n];
    dp[0][0] = 0;
    dp[0][1] = a[0];
    for i in 1..n {
        dp[i][0] = 0;
        for j in 1..=i + 1 {
            dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - 1] + j as i64 * a[i]);
        }
    }
    println!("{}", dp[n - 1][m]);
}
