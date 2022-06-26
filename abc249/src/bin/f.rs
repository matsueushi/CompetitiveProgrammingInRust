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
    let mut max_add = 0;
    let mut res = std::i64::MIN;
    let mut drop_one = 0;
    for (t, y) in ty.iter().rev() {
        if *t == 1 {
            if drop_one > k {
                break;
            } else {
                drop_one += 1;
                res = cmp::max(res, y + max_add);
            }
        } else {
            if *y >= 0 {
                max_add += y;
            } else {
                neg_cost.push(y);
                while !(neg_cost.is_empty()) && neg_cost.len() + drop_one > k {
                    let neg = neg_cost.pop().unwrap();
                    max_add += neg;
                }
            }
        }
    }
    if drop_one <= k {
        res = cmp::max(res, max_add);
    }
    println!("{}", res);
}
