use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![0; 2]; n + 1];
    for i in 0..n {
        dp[i + 1][0] = dp[i][1];
        dp[i + 1][1] = dp[i][1].max(dp[i][0] + a[i]);
    }
    println!("{}", dp[n][0].max(dp[n][1]));
}
