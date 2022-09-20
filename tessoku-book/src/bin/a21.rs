use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        pa: [(Usize1, usize); n],
    }

    let mut dp = vec![vec![0; n]; n];

    for d in (1..n).rev() {
        for i in 0..n - d {
            let j = i + d;
            let (pi, ai) = &pa[i];
            let (pj, aj) = &pa[j];
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
            dp[i][j - 1] = dp[i][j - 1].max(dp[i][j]);
            if i <= *pi && *pi <= j {
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j] + ai);
            }
            if i <= *pj && *pj <= j {
                dp[i][j - 1] = dp[i][j - 1].max(dp[i][j] + aj);
            }
        }
    }

    let mut res = 0;
    for i in 0..n {
        res = res.max(dp[i][i]);
    }
    println!("{}", res);
}
