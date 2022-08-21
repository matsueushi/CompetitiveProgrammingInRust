use proconio::{fastout, input};

const M: usize = 100_000;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v: Vec<i64> = vec![0; M];
    for ai in a {
        v[ai] += 1;
    }

    let mut res = 0;
    for i in 1..M / 2 {
        res += v[i] * v[M - i];
    }
    let v = v[M / 2];
    res += (v * (v - 1)) / 2;
    println!("{}", res);
}
