use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let slen = s.len();
    let tlen = t.len();
    let mut dp = vec![vec![0; tlen + 1]; slen + 1];
    for i in 1..=slen {
        for j in 1..=tlen {
            dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] + 1);
            }
        }
    }
    println!("{}", dp[slen][tlen]);
}
