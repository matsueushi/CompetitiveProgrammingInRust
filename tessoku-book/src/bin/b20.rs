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
    for i in 0..=slen {
        dp[i][0] = i;
    }
    for j in 0..=tlen {
        dp[0][j] = j;
    }
    for i in 1..=slen {
        for j in 1..=tlen {
            let cost = if s[i - 1] == t[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }
    println!("{}", dp[slen][tlen]);
}
