use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        cs: [usize; n],
    }
    let mut xs = vec![0; n];
    for i in 0..n {
        for k in 0..3 {
            let idx = (n + i - cs[i] + k - 1) % n;
            xs[idx] += 1;
        }
    }
    println!("{}", xs.iter().max().unwrap());
}
