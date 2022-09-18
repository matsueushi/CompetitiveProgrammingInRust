use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 1..=n {
        for j in 0..=s {
            dp[i][j] = dp[i - 1][j];
            if j >= a[i - 1] {
                dp[i][j] |= dp[i - 1][j - a[i - 1]];
            }
        }
    }
    println!("{}", if dp[n][s] { "Yes" } else { "No" });
}
