use std::cmp::Ordering;

use proconio::{fastout, input};

fn search(a: &Vec<usize>, x: usize) -> Option<usize> {
    let mut l = 0;
    let mut r = a.len();
    while l < r {
        let m = (l + r) / 2;
        match x.cmp(&a[m]) {
            Ordering::Less => {
                r = m;
            }
            Ordering::Equal => {
                return Some(m);
            }
            Ordering::Greater => {
                l = m;
            }
        }
    }
    None
}

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    println!("{}", search(&a, x).unwrap() + 1);
}
