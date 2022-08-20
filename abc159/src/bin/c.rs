use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: usize,
    }
    let l = (l as f64) / 3.0;
    println!("{}", l * l * l);
}
