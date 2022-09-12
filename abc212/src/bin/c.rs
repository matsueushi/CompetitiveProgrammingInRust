use proconio::{fastout, input};
use std::cmp::Ordering::{Greater, Less};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        mut b: [i64; m],
    }
    b.sort();

    let mut res = std::i64::MAX;
    for ai in a {
        let i = b
            .binary_search_by(|&x| if !(x >= ai) { Less } else { Greater })
            .unwrap_or_else(|i| i);
        if i != b.len() {
            res = res.min((ai - b[i]).abs());
        }
        if i > 0 {
            res = res.min((ai - b[i - 1]).abs());
        }
    }
    println!("{}", res);
}
