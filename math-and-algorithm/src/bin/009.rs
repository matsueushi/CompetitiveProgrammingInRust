use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let mut dp = vec![false; s + 1];
    dp[0] = true;
    for ai in a {
        let mut dp2 = dp.clone();
        for j in 0..=s {
            if j + ai <= s {
                dp2[j + ai] |= dp[j];
            }
        }
        std::mem::swap(&mut dp, &mut dp2);
    }
    println!("{}", if dp[s] { "Yes" } else { "No" });
}
