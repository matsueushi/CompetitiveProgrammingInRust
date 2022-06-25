use core::cmp::Ordering::{Greater, Less};
use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut as_: [usize; n],
        xs: [usize; q],
    }
    as_.sort();

    let bs: Vec<usize> = as_.iter().cumsum().collect();
    let asum = bs.last().unwrap();
    for x in xs {
        let pos = as_
            .binary_search_by(|&y| if y <= x { Less } else { Greater })
            .unwrap_or_else(|i| i);
        if pos == 0 {
            println!("{}", asum - n * x);
        } else {
            let ops = pos * x + asum - 2 * bs[pos - 1] - (n - pos) * x;
            println!("{}", ops)
        }
    }
}
