use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut res: Vec<usize> = vec![0; n];
    for i in 0..n - 1 {
        if a[i] > a[i + 1] {
            res[i] ^= 1;
            res[i + 1] ^= 1;
        }
    }
    println!("{}", res.into_iter().map(|x| x.to_string()).join(" "));
}
