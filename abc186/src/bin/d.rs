use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    a.sort();
    let mut res = 0;
    for i in 0..n {
        let (m1, m2) = (i as i64, (n - 1 - i) as i64);
        res += (m1 - m2) * a[i];
    }
    println!("{}", res);
}
