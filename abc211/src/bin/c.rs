use proconio::{fastout, input, marker::Chars};

const P: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let chokudai: Vec<char> = r"chokudai".chars().collect();
    let mut dp = vec![vec![0; chokudai.len()]; s.len() + 1];
    for i in 0..s.len() {
        for k in 0..chokudai.len() {
            dp[i + 1][k] = dp[i][k];
            if s[i] == chokudai[k] {
                if k == 0 {
                    dp[i + 1][k] += 1;
                    dp[i + 1][k] %= P;
                } else {
                    dp[i + 1][k] += dp[i][k - 1];
                    dp[i + 1][k] %= P;
                }
            }
        }
    }
    println!("{}", dp[s.len()][chokudai.len() - 1]);
}
