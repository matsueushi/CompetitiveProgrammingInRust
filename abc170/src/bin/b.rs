use proconio::{fastout, input};

fn solve(x: i64, y: i64) -> bool {
    if y % 2 != 0 {
        return false;
    }
    let a = y / 2 - x;
    let b = x - a;
    a >= 0 && b >= 0
}

#[fastout]
fn main() {
    input! {
        x: i64,
        y: i64,
    }
    println!("{}", if solve(x, y) { "Yes" } else { "No" });
}
