use proconio::{fastout, input};

fn solve(r: i64, c: i64) -> usize {
    if r == 0 && c == 0 {
        0
    } else if r + c == 0 || r - c == 0 || r.abs() + c.abs() <= 3 {
        1
    } else if (r + c) % 2 == 0 || (r + c).abs() <= 3 || (r - c).abs() <= 3 || r.abs() + c.abs() <= 6
    {
        2
    } else {
        3
    }
}

#[fastout]
fn main() {
    input! {
        r1: i64,
        c1: i64,
        r2: i64,
        c2: i64,
    }
    let r = r2 - r1;
    let c = c2 - c1;
    println!("{}", solve(r, c));
}
