use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 0..n {
        for j in 1..=2 {
            if i + j <= n {
                dp[i + j] += dp[i];
            }
        }
    }
    println!("{}", dp[n]);
}
