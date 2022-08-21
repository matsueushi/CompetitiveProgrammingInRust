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

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    println!("{}", pow_mod(2, n, 10));
}
