use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
    }
    let c = w / 2;
    let res = h * c + (w % 2) * ((h + 1) / 2);
    println!("{}", res);
}
