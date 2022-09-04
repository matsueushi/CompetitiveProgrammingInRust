use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    }

    let mut c = vec![0];
    for i in 0..a.len() {
        c.push(c[i] + a[i]);
    }

    // start
    let mut x = 0;
    for i in 0..m {
        x += (i + 1) as i64 * a[i];
    }
    let mut res = x;

    for k in 1..=n - m {
        x += m as i64 * a[m - 1 + k] - c[k + m - 1] + c[k - 1];
        res = res.max(x);
    }
    println!("{}", res);
}
