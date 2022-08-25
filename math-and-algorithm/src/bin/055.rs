use num::Integer;
use proconio::{fastout, input};
use std::ops::{AddAssign, RemAssign};
pub type Matrix<T> = Vec<Vec<T>>;

pub fn matmul<T>(a: &Matrix<T>, b: &Matrix<T>, p: T) -> Matrix<T>
where
    T: Copy + Clone + Integer + AddAssign + RemAssign,
{
    let mut c = vec![vec![T::zero(); b[0].len()]; a.len()];
    for i in 0..a.len() {
        for k in 0..b.len() {
            for j in 0..b[0].len() {
                c[i][j] += a[i][k] * b[k][j];
                c[i][j] %= p;
            }
        }
    }
    c
}

pub fn matpow<T>(a: &Matrix<T>, n: usize, p: T) -> Matrix<T>
where
    T: Copy + Clone + Integer + AddAssign + RemAssign,
{
    let mut a = a.clone();
    let mut b = vec![vec![T::zero(); a.len()]; a.len()];
    for i in 0..a.len() {
        b[i][i] = T::one();
    }
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            b = matmul(&b, &a, p);
        }
        a = matmul(&a, &a, p);
        n >>= 1;
    }
    b
}

const P: usize = 1_000_000_007;
#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let a: Matrix<usize> = vec![vec![2, 1], vec![1, 0]];
    let a = matpow(&a, n - 1, P);
    println!("{}", matmul(&a, &vec![vec![1], vec![1]], P)[1][0]);
}
