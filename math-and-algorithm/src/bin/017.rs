use proconio::{fastout, input};

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    let g = gcd(a, b);
    (a / g) * b
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut l = lcm(a[0], a[1]);
    for i in 2..n {
        l = lcm(l, a[i]);
    }
    println!("{}", l);
}
