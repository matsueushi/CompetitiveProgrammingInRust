use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-2],
    }
    let mut dp = vec![std::usize::MAX; n];
    dp[0] = 0;
    for i in 0..n - 1 {
        dp[i + 1] = dp[i + 1].min(dp[i] + a[i]);
        if i != n - 2 {
            dp[i + 2] = dp[i + 2].min(dp[i] + b[i]);
        }
    }

    println!("{}", dp[n - 1]);
}
