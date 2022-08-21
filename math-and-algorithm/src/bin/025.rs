use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut res = 0.0;
    res += a.iter().sum::<usize>() as f64 / 3.0;
    res += b.iter().sum::<usize>() as f64 * 2.0 / 3.0;
    println!("{}", res);
}
