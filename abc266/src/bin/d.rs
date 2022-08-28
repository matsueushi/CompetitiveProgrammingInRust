use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        txa: [(usize, usize, i64); n],
    }

    let (max_t, _, _) = txa.last().unwrap();

    let mut snuke = HashMap::new();
    for (t, x, a) in &txa {
        snuke.insert(*t, (*x, *a));
    }

    let mut dp = vec![vec![std::i64::MIN; 5]; max_t + 1];
    dp[0][0] = 0;

    for i in 0..*max_t {
        for j in 0..5 {
            dp[i + 1][j] = dp[i][j];
            if j != 4 {
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j + 1]);
            }
            if j != 0 {
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j - 1]);
            }
        }

        if let Some((x, a)) = snuke.get(&(i + 1)) {
            dp[i + 1][*x] += *a;
        };
    }

    let mut res = 0;
    for v in &dp[*max_t] {
        res = res.max(*v);
    }
    println!("{}", res);
}
