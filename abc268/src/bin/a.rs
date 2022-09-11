use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
       xs: [usize; 5] ,
    }

    let xset: HashSet<_> = xs.into_iter().collect();
    println!("{}", xset.len());
}
