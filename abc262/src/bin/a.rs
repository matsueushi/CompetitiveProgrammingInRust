use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        y: usize,
    }
    let mut x = 2 + (y / 4) * 4;
    if x < y {
        x += 4;
    }
    println!("{}", x);
}
