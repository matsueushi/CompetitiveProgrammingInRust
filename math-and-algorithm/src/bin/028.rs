use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }
    let mut cs = vec![std::i64::MAX; n];
    cs[0] = 0;
    for i in 0..n {
        for j in 1..=2 {
            if i + j < n {
                cs[i + j] = cs[i + j].min(cs[i] + (h[i] - h[i + j]).abs());
            }
        }
    }
    println!("{}", cs[n - 1]);
}
