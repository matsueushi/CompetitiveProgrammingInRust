use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        r: usize,
        c: usize,
    }

    fn abs_diff(a: usize, b: usize) -> usize {
        if a > b {
            a - b
        } else {
            b - a
        }
    }

    let d = abs_diff(r, 8).max(abs_diff(c, 8));
    println!("{}", if d % 2 == 0 { "white" } else { "black" });
}
