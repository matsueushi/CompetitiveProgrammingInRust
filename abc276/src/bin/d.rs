use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n : usize,
        a: [usize; n],
    }
    let mut d2 = Vec::new();
    let mut d3 = Vec::new();
    let mut b = HashSet::new();
    for ai in a {
        let mut x = 0;
        let mut u = ai;
        while u % 2 == 0 {
            u /= 2;
            x += 1;
        }
        d2.push(x);

        let mut x = 0;
        while u % 3 == 0 {
            u /= 3;
            x += 1;
        }
        d3.push(x);

        b.insert(u);
    }
    if b.len() != 1 {
        println!("-1");
    } else {
        let d2min = d2.iter().min().unwrap();
        let d2sum = d2.iter().sum::<usize>();
        let d3min = d3.iter().min().unwrap();
        let d3sum = d3.iter().sum::<usize>();
        println!("{}", d2sum - d2min * n + d3sum - d3min * n);
    }
}
