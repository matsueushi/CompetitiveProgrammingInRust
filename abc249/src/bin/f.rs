use std::cmp;
use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ty: [(usize, i64); n],
    }
    let mut neg_cost = BinaryHeap::new();
    let mut skip_one = 0;
    let mut max_add = 0;
    let mut res = 0;
    for (t, y) in ty.iter().rev() {
        println!("{} {}", t, y);
        if t == 1 {
            skip_one += 1;
            if skip_one > k {
                continue;
            } else {
            }
        } else {
            if y >= 0 {
                max_add += y;
            } else {
                neg_cost.push(y);
                if neg_cost.len() > k {
                    let neg = neg_cost.pop().unwrap();
                    max_add -= neg;
                }
            }
        }
    }
    if skip_one <= k {
        res = cmp::max(res, t + max_add);
    }
    println!("{}", res);
}
