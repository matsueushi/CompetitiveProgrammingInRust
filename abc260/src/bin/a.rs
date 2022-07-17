use proconio::{fastout, input, marker::Chars};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut h = HashMap::new();
    for x in s {
        h.entry(x).or_insert(0);
        h.insert(x, h.get(&x).unwrap() + 1);
    }

    for (k, v) in &h {
        if v == &1 {
            println!("{}", k);
            return;
        }
    }
    println!("{}", -1);
}
