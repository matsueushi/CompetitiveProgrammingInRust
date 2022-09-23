use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: f64,
    }
    let bi = (b * 100.0).round() as usize;
    let au = a / 100;
    let al = a % 100;
    println!("{}", au * bi + (al * bi / 100));
}
