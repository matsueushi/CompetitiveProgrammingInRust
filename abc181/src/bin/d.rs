use proconio::{fastout, input, marker::Chars};
use std::collections::HashMap;

fn solve(s: &Vec<u32>) -> bool {
    if s.len() == 1 {
        return s == &vec![8];
    }

    let mut counter = HashMap::new();
    for c in s {
        *counter.entry(c).or_insert(0) += 1;
    }

    let (st, en) = if s.len() == 2 { (11, 100) } else { (111, 1000) };

    for i in st..en {
        if i % 8 != 0 {
            continue;
        }
        let mut y = HashMap::new();
        let mut j = i;
        while j > 0 {
            let x = j % 10;
            j /= 10;
            *y.entry(x).or_insert(0) += 1;
        }

        let mut flag = true;
        for (k, v) in y {
            match counter.get(&k) {
                Some(&v0) => {
                    if v0 < v {
                        flag = false;
                    }
                }
                None => flag = false,
            }
        }
        if flag {
            return true;
        }
    }
    false
}

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let s = s.into_iter().map(|x| x.to_digit(10).unwrap()).collect();
    println!("{}", if solve(&s) { "Yes" } else { "No" });
}
