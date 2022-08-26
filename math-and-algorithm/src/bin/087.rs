use proconio::{fastout, input};

const P: usize = 1000000007;

fn f(x: usize) -> usize {
    ((x % P) * ((x + 1) % P) / 2) % P
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let x = f(n) * f(n) % P;
    println!("{}", x);
}
