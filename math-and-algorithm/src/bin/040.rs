use proconio::{fastout, input, marker::Usize1};
use std::ops::Sub;

fn abs_diff<T>(a: T, b: T) -> T
where
    T: Sub<Output = T> + PartialOrd,
{
    if a > b {
        a - b
    } else {
        b - a
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        m: usize,
        b: [Usize1; m],
    }
    let mut v = vec![0];
    for i in 0..n - 1 {
        v.push(v[i] + a[i]);
    }
    let mut res = 0;
    for i in 1..m {
        res += abs_diff(v[b[i]], v[b[i - 1]]);
    }
    println!("{}", res);
}
