use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q],
    }
    let mut csum = vec![0];
    for i in 0..n {
        csum.push(csum[i] + a[i]);
    }
    for (l, r) in &lr {
        println!("{}", csum[*r] - csum[*l - 1]);
    }
}
