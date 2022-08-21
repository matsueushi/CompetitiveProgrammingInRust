use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
    }
    let mut res = 0;
    for i in 1..=n.min(s - 1) {
        res += (s - i).min(n);
    }
    println!("{}", res);
}
