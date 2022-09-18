use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![false; s + 1]; n + 1];
    let mut prev = vec![vec![0; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 1..=n {
        for j in 0..=s {
            dp[i][j] = dp[i - 1][j];
            if j >= a[i - 1] {
                dp[i][j] |= dp[i - 1][j - a[i - 1]];
            }

            if dp[i][j] != dp[i - 1][j] {
                prev[i][j] = j - a[i - 1];
            } else {
                prev[i][j] = j;
            }
        }
    }

    if dp[n][s] {
        let mut path = Vec::new();
        let mut j = s;
        for i in (1..=n).rev() {
            if prev[i][j] != j {
                j = prev[i][j];
                path.push(i);
            }
        }
        path.reverse();
        println!("{}", path.len());
        println!(
            "{}",
            path.into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    } else {
        println!("-1");
    }
}
