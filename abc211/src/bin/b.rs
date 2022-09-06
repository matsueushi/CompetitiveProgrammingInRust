use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        ss: [String; 4],
    }
    let ss: HashSet<_> = ss.into_iter().collect();
    if ss.len() == 4 {
        println!("Yes");
    } else {
        println!("No");
    }
}
