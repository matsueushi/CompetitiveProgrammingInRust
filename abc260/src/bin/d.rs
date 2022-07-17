use proconio::{fastout, input};
use std::collections::BTreeSet;
use std::ops::Bound::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }

    let mut eat = vec![-1; n + 1];
    let mut next = vec![std::usize::MAX; n + 1];
    let mut nums = vec![0; n + 1];
    let mut top = BTreeSet::new();

    for i in 0..n {
        let card = p[i];
        let mut after = top.range((Excluded(card), Unbounded));
        match after.next() {
            None => {
                nums[card] = 1;
            }
            Some(&pos) => {
                nums[card] = nums[pos] + 1;
                next[card] = pos;
                top.remove(&pos);
            }
        }

        if nums[card] == k {
            let mut j = card;
            loop {
                eat[j] = i as i64 + 1;
                if next[j] == std::usize::MAX {
                    break;
                }
                j = next[j];
            }
        } else {
            top.insert(card);
        }
    }

    for i in 1..n + 1 {
        println!("{}", eat[i]);
    }
}
