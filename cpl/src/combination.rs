pub mod combination {
    pub struct PrimeModCombination {
        p: u64,
        fact: Vec<u64>,
        fact_inv: Vec<u64>,
    }

    impl PrimeModCombination {
        pub fn new(n: usize, p: u64) -> Self {
            // i! mod p
            let mut fact = vec![0; n + 1];
            // (i!)^{-1} mod p
            let mut fact_inv = vec![0; n + 1];
            // i^{-1} mod p (i > 0)
            let mut iinv = vec![0; n + 1];
            fact[0] = 1;
            fact[1] = 1;
            fact_inv[0] = 1;
            fact_inv[1] = 1;
            iinv[1] = 1;
            for i in 2..n + 1 {
                let j = i as u64;
                fact[i] = fact[i - 1] * j % p;
                iinv[i] = p - iinv[(p % j) as usize] * (p / j) % p;
                fact_inv[i] = fact_inv[i - 1] * iinv[i] % p;
            }
            Self { p, fact, fact_inv }
        }

        pub fn combination(&self, n: usize, k: usize) -> u64 {
            self.fact[n] * (self.fact_inv[k] * self.fact_inv[n - k] % self.p) % self.p
        }
    }
}

#[cfg(test)]
mod tests {
    use super::combination::*;

    #[test]
    fn test_power_mod_combination() {
        let pmc = PrimeModCombination::new(10, 7);
        assert_eq!(pmc.combination(3, 2), 3);
        assert_eq!(pmc.combination(6, 3), 6);
    }
}
