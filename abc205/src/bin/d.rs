use proconio::{fastout, input, marker::Usize1};
use std::cmp::Ordering::{Greater, Less};

#[fastout]
fn main() {
    input! {
       n: usize,
       q: usize,
       a: [usize; n],
       k: [Usize1; q],
    }
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    let mut i = 0;
    while i < n {
        if i == 0 && a[0] != 1 {
            xs.push(1);
            ys.push(0);
        }
        if i == n - 1 || a[i] + 1 != a[i + 1] {
            xs.push(a[i] + 1);
            ys.push(a[i] - i - 1);
        }
        i += 1;
    }

    for ki in k {
        let i = ys
            .binary_search_by(|&x| if x <= ki { Less } else { Greater })
            .unwrap_or_else(|i| i)
            - 1;
        println!("{}", xs[i] + ki - ys[i]);
    }
}
