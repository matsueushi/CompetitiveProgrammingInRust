pub mod power_mod {
    pub fn pow_mod(x: u64, n: u64, m: u64) -> u64 {
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
}

#[cfg(test)]
mod tests {
    use super::power_mod::*;

    #[test]
    fn test_pow_mod() {
        assert_eq!(pow_mod(5, 3, 19), 11);
        assert_eq!(pow_mod(2, 2, 998244353), 4);
        assert_eq!(pow_mod(2, 3, 7), 1);
        assert_eq!(pow_mod(1, 1, 998244353), 1);
    }
}
