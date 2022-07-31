use proconio::{fastout, input};

const P: usize = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut res = 0;
    for k in 1..=n {
        // dp[i, j, l]
        let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; k]; k + 1]; n + 1];
        dp[0][0][0] = 1;
        dp[0][1][a[0] % k] = 1;
        for i in 1..n {
            for j in 0..k + 1 {
                for l in 0..k {
                    dp[i][j][l] += dp[i - 1][j][l];
                    dp[i][j][l] %= P;
                    if j > 0 {
                        dp[i][j][(l + a[i]) % k] += dp[i - 1][j - 1][l];
                        dp[i][j][(l + a[i]) % k] %= P;
                    }
                }
            }
        }
        res += dp[n - 1][k][0];
        res %= P;
    }
    println!("{}", res);
}
