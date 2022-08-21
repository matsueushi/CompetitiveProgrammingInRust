use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        b: [usize; n],
        r: [usize; n],
    }
    let res = b.iter().sum::<usize>() + r.iter().sum::<usize>();
    println!("{}", (res as f64) / (n as f64));
}
