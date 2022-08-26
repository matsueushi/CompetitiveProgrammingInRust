use proconio::{fastout, input};

const P: usize = 998244353;

fn f(x: usize) -> usize {
    ((x % P) * ((x + 1) % P) / 2) % P
}

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let x = ((f(a) * f(b)) % P) * f(c) % P;
    println!("{}", x);
}
