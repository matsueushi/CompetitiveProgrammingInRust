use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        v: [usize; n],
        c: [usize; n],
    }
    let mut res = 0;
    for i in 0..n {
        if v[i] > c[i] {
            res += v[i] - c[i];
        }
    }
    println!("{}", res);
}
