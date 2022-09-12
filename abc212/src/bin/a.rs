use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    if 0 < a && b == 0 {
        println!("Gold");
    } else if a == 0 && 0 < b {
        println!("Silver");
    } else {
        println!("Alloy");
    }
}
