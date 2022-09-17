// shakutori

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut rs = vec![0; n];
    for i in 0..n - 1 {
        if i == 0 {
            rs[i] = 0;
        } else {
            rs[i] = rs[i - 1];
        }

        while rs[i] < n - 1 && a[rs[i] + 1] - a[i] <= k {
            rs[i] += 1;
        }
    }

    let mut res = 0;
    for i in 0..n - 1 {
        res += rs[i] - i;
    }
    println!("{}", res);
}
