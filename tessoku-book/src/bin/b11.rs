use proconio::{fastout, input};
use std::cmp::Ordering::{Greater, Less};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
        x: [usize; q],
    }
    a.sort();

    for xi in x {
        let i = a
            .binary_search_by(|&x| if x < xi { Less } else { Greater })
            .unwrap_or_else(|i| i);
        println!("{}", i);
    }
}
