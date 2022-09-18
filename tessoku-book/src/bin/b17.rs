use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }
    let mut dp = vec![std::i64::MAX; n];
    let mut prev = vec![0; n];
    dp[0] = 0;
    for i in 1..n {
        dp[i] = dp[i - 1] + (h[i] - h[i - 1]).abs();
        prev[i] = i - 1;
        if i != 1 {
            let d = dp[i - 2] + (h[i] - h[i - 2]).abs();
            if dp[i] > d {
                prev[i] = i - 2;
                dp[i] = d;
            }
        }
    }
    let mut j = n - 1;
    let mut path = Vec::new();
    while j != 0 {
        path.push(j);
        j = prev[j];
    }
    path.push(j);

    path.reverse();
    println!("{}", path.len());
    for i in 0..path.len() {
        print!("{}", path[i] + 1);
        if i != path.len() - 1 {
            print!(" ");
        }
    }
    println!();
}
