use proconio::{fastout, input};

const P: usize = 998244353;

fn f(m: usize, u: usize, v: usize) -> usize {
    let m = m % P;
    let u = u % P;
    let v = v % P;
    let vv = v * v % P;
    let uvv = u * vv % P;
    let uuu = (u * (P + u - 1)) % P;
    let mv = m * v % P;
    let uumv = uuu * mv % P;
    (uvv + uumv) % P
}

fn g(m: usize, x: usize, y: usize) -> usize {
    if x == 0 || y == 0 {
        return 0;
    }
    let mut res = f(m, (x + 1) / 2, (y + 1) / 2);
    if x >= 2 && y >= 2 {
        let res2 = f(m, x / 2, y / 2);
        let mut add = (x / 2) * (y / 2) % P;
        add = (add * (m + 1)) % P;
        res = (res + res2 + add) % P;
    }
    res
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }
    for (a, b, c, d) in abcd {
        let res = (2 * P + g(m, b, d) + g(m, a - 1, c - 1) - g(m, b, c - 1) - g(m, a - 1, d)) % P;
        println!("{}", res);
    }
}
