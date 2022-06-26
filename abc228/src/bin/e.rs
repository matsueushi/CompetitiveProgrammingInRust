use proconio::input;

fn pow_mod(x: u64, n: u64, m: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    let mut x = x % m;
    let mut res = 1;
    let mut p = n;
    while p > 0 {
        if p & 1 != 0 {
            res *= x;
            res %= m;
        }
        x *= x;
        x %= m;
        p >>= 1;
    }
    res
}

const P: u64 = 998244353;

fn main() {
    input! {
        n: u64,
        k: u64,
        m: u64,
    }
    if m % P == 0 {
        println!("0")
    } else {
        let b = pow_mod(k, n, P - 1);
        let res = pow_mod(m, b, P);
        println!("{}", res);
    }
}
