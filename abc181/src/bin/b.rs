use proconio::{fastout, input};

fn c2(u: usize) -> usize {
    u * (u + 1) / 2
}

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut res = 0;
    for (a, b) in &ab {
        res += c2(*b);
        res -= c2(*a - 1);
    }
    println!("{}", res);
}
