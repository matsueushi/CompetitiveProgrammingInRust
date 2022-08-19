pub mod power_mod {
    use std::ops::{MulAssign, RemAssign, ShrAssign};
    use num::{PrimInt, Unsigned};

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

#[cfg(test)]
mod tests {
    use super::power_mod::*;

    #[test]
    fn test_pow_mod() {
        assert_eq!(pow_mod::<u32>(5, 3, 19), 11);
        assert_eq!(pow_mod::<u64>(2, 3, 7), 1);
        assert_eq!(pow_mod::<usize>(2, 2, 998244353), 4);
        assert_eq!(pow_mod::<usize>(1, 1, 998244353), 1);
    }
}
