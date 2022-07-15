use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xs: [usize; n],
    }
    let mut indices = (0..n).collect_vec();
    indices.sort_by_key(|&i| &xs[i]);
    println!("{}", &indices[n - 2] + 1);
}
