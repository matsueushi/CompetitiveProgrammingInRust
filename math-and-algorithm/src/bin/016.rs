use proconio::{fastout, input};

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut g = gcd(a[0], a[1]);
    for i in 2..n {
        g = gcd(g, a[i]);
    }
    println!("{}", g);
}
