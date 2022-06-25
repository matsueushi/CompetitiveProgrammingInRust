use num::integer::Integer;
use proconio::input;
use std::cmp;

fn solve(x: i64, a: i64, d: i64, n: i64) -> i64 {
    if d == 0 {
        return (x - a).abs();
    } else if (d > 0 && x - a < 0) || (d < 0 && x - a > 0) {
        (x - a).abs()
    } else if (d > 0 && x - a > d * (n - 1)) || (d < 0 && x - a < d * (n - 1)) {
        (x - (a + d * (n - 1))).abs()
    } else {
        let up = (x - a).div_ceil(&d);
        let dw = (x - a).div_floor(&d);
        let r1 = (x - (a + d * up)).abs();
        let r2 = (x - (a + d * dw)).abs();
        cmp::min(r1, r2)
    }
}

fn main() {
    input! {
        x: i64,
        a: i64,
        d: i64,
        n: i64,
    }
    let res = solve(x, a, d, n);
    println!("{}", res)
}
