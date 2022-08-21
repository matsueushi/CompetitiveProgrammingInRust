use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        pq: [(usize, usize); n],
    }
    let mut score = 0.0;
    for (p, q) in &pq {
        score += (*q as f64) / (*p as f64);
    }
    println!("{}", score);
}
