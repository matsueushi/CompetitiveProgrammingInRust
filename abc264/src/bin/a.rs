use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: usize,
        r: usize,
    }
    let s = ['a', 't', 'c', 'o', 'd', 'e', 'r'];
    println!(
        "{}",
        s[l - 1..r].into_iter().map(|x| x.to_string()).join("")
    );
}
