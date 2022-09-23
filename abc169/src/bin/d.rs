use proconio::{fastout, input};
use std::cmp::Ordering::{Greater, Less};
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
    let mut count = Vec::new();
    for i in 0..100 {
        count.push(i * (i + 1) / 2);
    }
    let factors = prime_factor(n);
    let mut res = 0;
    for (_, e) in &factors {
        let i = count
            .binary_search_by(|&x| if x <= *e { Less } else { Greater })
            .unwrap_or_else(|i| i)
            - 1;
        res += i;
    }
    println!("{}", res);
}
