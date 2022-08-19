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
    let mut res = pow_mod(10, n, P);
    res += 2 * (P - pow_mod(9, n, P));
    res %= P;
    res += pow_mod(8, n, P);
    res %= P;
    println!("{}", res);
}
