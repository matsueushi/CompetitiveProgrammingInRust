pub mod matrix {
    use std::ops::{AddAssign, RemAssign};

    use num::Integer;

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
}

#[cfg(test)]
mod tests {
    use super::matrix::*;

    #[test]
    fn test_matmul_0() {
        let a = vec![vec![1, 0], vec![0, 1]];
        let b = vec![vec![4, 1], vec![2, 2]];
        assert_eq!(matmul(&a, &b, 100), b);
    }

    #[test]
    fn test_matmul_1() {
        let a = vec![vec![1, 0], vec![0, 1]];
        let b = vec![vec![1], vec![2]];
        assert_eq!(matmul(&a, &b, 100), b);
    }

    #[test]
    fn test_matpow() {
        let a = vec![vec![2, 0], vec![0, 2]];
        assert_eq!(matpow(&a, 2, 10), vec![vec![4, 0], vec![0, 4]]);
    }
}
