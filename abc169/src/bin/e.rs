use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut a = Vec::new();
    let mut b = Vec::new();
    for (ai, bi) in ab {
        a.push(ai);
        b.push(bi);
    }
    a.sort();
    b.sort();
    if n % 2 == 0 {
        let mmin = a[n / 2 - 1] + a[n / 2];
        let mmax = b[n / 2 - 1] + b[n / 2];
        println!("{}", mmax - mmin + 1);
    } else {
        println!("{}", b[n / 2] - a[n / 2] + 1);
    }
}
