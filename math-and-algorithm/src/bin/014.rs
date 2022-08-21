use proconio::{fastout, input};
use std::collections::HashMap;

pub fn prime_factor(n: usize) -> HashMap<usize, usize> {
    let mut res = HashMap::new();
    let mut i = 1;
    let mut x = n;
    loop {
        i += 1;
        if i * i > n {
            break;
        }
        while x % i == 0 {
            *res.entry(i).or_insert(0) += 1;
            x /= i;
        }
    }
    if x != 1 {
        res.insert(x, 1);
    }
    res
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let fs = prime_factor(n);
    let mut res = Vec::new();
    for (k, v) in &fs {
        for _ in 0..*v {
            res.push(k);
        }
    }
    res.sort();
    println!(
        "{}",
        res.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
