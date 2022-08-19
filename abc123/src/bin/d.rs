use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        k: usize,
        a: [usize; x],
        b: [usize; y],
        c: [usize; z],
    }
    let mut d = Vec::new();
    for ai in &a {
        for bi in &b {
            d.push(ai + bi);
        }
    }
    d.sort_by_key(|&w| std::cmp::Reverse(w));
    let mut d2 = Vec::new();
    for i in 0..(x * y).min(k) {
        for ci in &c {
            d2.push(d[i] + ci);
        }
    }
    d2.sort_by_key(|&w| std::cmp::Reverse(w));
    for i in 0..k {
        println!("{}", d2[i]);
    }
}
