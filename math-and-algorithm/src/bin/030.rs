use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }
    let mut dp = vec![0; w + 1];
    for (wi, vi) in &wv {
        let mut dp2 = dp.clone();
        for j in 0..w {
            if j + wi > w {
                break;
            }
            dp2[j + wi] = dp2[j + wi].max(dp[j] + vi);
        }
        std::mem::swap(&mut dp, &mut dp2);
    }
    println!("{}", dp[w]);
}
