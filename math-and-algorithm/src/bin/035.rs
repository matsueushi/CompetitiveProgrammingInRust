use num::Integer;
use proconio::{fastout, input};

pub fn dist2<T>(x0: T, y0: T, x1: T, y1: T) -> T
where
    T: Copy + Integer,
{
    let dx = x0 - x1;
    let dy = y0 - y1;
    dx * dx + dy * dy
}

#[fastout]
fn main() {
    input! {
        x1: i64,
        y1: i64,
        r1: i64,
        x2: i64,
        y2: i64,
        r2: i64,
    }
    let d2 = dist2(x1, y1, x2, y2);
    let rr1 = (r1 - r2).pow(2);
    let rr2 = (r1 + r2).pow(2);
    let res = if d2 < rr1 {
        1
    } else if d2 == rr1 {
        2
    } else if d2 < rr2 {
        3
    } else if d2 == rr2 {
        4
    } else {
        5
    };
    println!("{}", res);
}
