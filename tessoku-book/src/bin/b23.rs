use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut dist = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in i + 1..n {
            let (xi, yi) = xy[i];
            let (xj, yj) = xy[j];
            let (dx, dy) = (xi - xj, yi - yj);
            let d = ((dx * dx + dy * dy) as f64).sqrt();
            dist[i][j] = d;
            dist[j][i] = d;
        }
    }

    let mut dp = vec![vec![std::f64::MAX / 2.0; n]; 1 << n];
    let mut start = vec![vec![0; n]; 1 << n];
    for j in 0..n {
        dp[0][j] = 0.0;
        start[0][j] = j;
    }

    for i in 0..1 << n {
        for j in 0..n {
            for k in 0..n {
                let i2 = i | (1 << k);
                let d = dp[i][j] + dist[j][k];
                if d < dp[i2][k] {
                    dp[i2][k] = d;
                    start[i2][k] = start[i][j];
                }
            }
        }
    }

    let mut res = std::f64::MAX;
    for j in 0..n {
        res = res.min(dp[(1 << n) - 1][j] + dist[start[(1 << n) - 1][j]][j]);
    }

    println!("{}", res);
}
