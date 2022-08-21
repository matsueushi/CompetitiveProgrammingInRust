use num::{PrimInt, Unsigned};
use proconio::{fastout, input};
use std::ops::{MulAssign, RemAssign, ShrAssign};

pub fn pow_mod<T>(x: T, n: T, m: T) -> T
where
    T: PrimInt + Unsigned + MulAssign + RemAssign + ShrAssign,
{
    if n == T::zero() {
        return T::one();
    }
    let mut x = x % m;
    let mut res = T::one();
    let mut p = n;
    while p > T::zero() {
        if p & T::one() != T::zero() {
            res *= x;
            res %= m;
        }
        x *= x;
        x %= m;
        p >>= T::one();
    }
    res
}

const P: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut iinv = vec![0; 4];
    iinv[1] = 1;
    for i in 2..=3 {
        iinv[i] = P - iinv[P % i] * (P / i) % P;
    }
    let mut res = (P + pow_mod(4, n + 1, P) - 1) % P;
    res = res * iinv[3] % P;
    println!("{}", res);
}
