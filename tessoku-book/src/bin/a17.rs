use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-2],
    }
    let mut dp = vec![0; n];
    let mut prev = vec![0; n];
    dp[1] = a[0];
    for i in 2..n {
        let w1 = dp[i - 1] + a[i - 1];
        let w2 = dp[i - 2] + b[i - 2];
        dp[i] = w1.min(w2);
        prev[i] = if w1 < w2 { i - 1 } else { i - 2 };
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
