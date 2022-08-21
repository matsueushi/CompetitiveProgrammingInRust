use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v: Vec<i64> = vec![0; 4];
    for ai in a {
        v[ai] += 1;
    }
    let mut res = 0;
    for i in 1..=3 {
        res += (v[i] * (v[i] - 1)) / 2;
    }
    println!("{}", res);
}
