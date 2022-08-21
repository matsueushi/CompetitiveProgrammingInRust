use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![0; 1000 + 1]; 5 + 1];
    dp[0][0] = 1;
    for ai in &a {
        let mut dp2 = dp.clone();
        for i in 0..=4 {
            for j in 0..1000 {
                if j + ai > 1000 {
                    continue;
                }
                dp2[i + 1][j + ai] += dp[i][j];
            }
        }
        std::mem::swap(&mut dp, &mut dp2);
    }
    println!("{}", dp[5][1000]);
}
