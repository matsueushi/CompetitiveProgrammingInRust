use num::Integer;
use proconio::{fastout, input};
use std::iter;

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    if n.is_odd() {
        return;
    }
    let mut chars = iter::repeat('(')
        .take(n / 2)
        .chain(iter::repeat(')').take(n / 2))
        .collect::<Vec<_>>();
    loop {
        println!("{}", chars.iter().collect::<String>());
        match chars.windows(3).rposition(|v| v == &['(', ')', ')']) {
            None => break,
            Some(i) => {
                chars.swap(i, i + 1);
                chars[i + 1..].sort();
            }
        }
    }
}
