use std::collections::{HashMap, HashSet};

use num::Integer;
use proconio::{fastout, input};

use power_mod::*;

pub mod power_mod {
    use num::{PrimInt, Unsigned};
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
}

const P: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }

    let mut xs = HashMap::new();
    let mut zero_count = 0;
    for (a, b) in &ab {
        let mut a = *a;
        let mut b = *b;
        let au = a.abs() as usize;
        let bu = b.abs() as usize;
        let g = au.gcd(&bu) as i64;
        if g != 0 {
            a /= g;
            b /= g;
        }
        if (a, b) == (0, 0) {
            zero_count += 1;
        } else {
            *xs.entry((a, b)).or_insert(0_usize) += 1;
        }
    }

    let mut res = 1;
    let mut used = HashSet::new();
    for (x, y) in xs.keys() {
        let mut x = *x;
        let mut y = *y;
        let mut count = vec![0; 2];
        for i in 0..4 {
            if used.contains(&(x, y)) {
                continue;
            }
            used.insert((x, y));
            if let Some(v) = xs.get(&(x, y)) {
                count[i % 2] += v;
            }
            let (u, v) = (-y, x);
            x = u;
            y = v;
        }
        let z = (pow_mod(2, count[0], P) + pow_mod(2, count[1], P) - 1) % P;
        res = res * z % P;
    }
    res = (res + zero_count + P - 1) % P;
    println!("{}", res);
}
