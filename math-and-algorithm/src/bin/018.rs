use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v = vec![0; 5];
    for ai in a {
        v[ai / 100] += 1;
    }
    let res = v[1] * v[4] + v[2] * v[3];
    println!("{}", res);
}
