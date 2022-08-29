use std::cmp::Ordering;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: u64,
    }
    if c % 2 == 0 {
        match a.abs().cmp(&b.abs()) {
            Ordering::Equal => {
                println!("=");
            }
            Ordering::Less => {
                println!("<");
            }
            Ordering::Greater => {
                println!(">");
            }
        }
    } else {
        match a.cmp(&b) {
            Ordering::Equal => {
                println!("=");
            }
            Ordering::Less => {
                println!("<");
            }
            Ordering::Greater => {
                println!(">");
            }
        }
    }
}
